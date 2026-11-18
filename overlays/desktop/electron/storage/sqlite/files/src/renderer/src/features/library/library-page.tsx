import {
  createColumnHelper,
  flexRender,
  getCoreRowModel,
  useReactTable,
} from "@tanstack/react-table";
import { useVirtualizer } from "@tanstack/react-virtual";
import { formatDistanceToNow } from "date-fns";
import { ArrowDown, ArrowUp, ExternalLink, Plus, Search, Trash2 } from "lucide-react";
import { useDeferredValue, useEffect, useMemo, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { useLocation, useNavigate, useParams } from "react-router";
import { toast } from "sonner";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Empty, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle } from "@/components/ui/empty";
import { Input } from "@/components/ui/input";
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from "@/components/ui/resizable";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Separator } from "@/components/ui/separator";
import type { LibraryItem, LibraryListInput, LibraryStatus } from "@shared/ipc/library";
import { DesktopApiError, invoke } from "@renderer/lib/desktop-api";
// smokeforge:anchor library-sync-imports
import { cn } from "@/lib/utils";

import { LibraryForm } from "./library-form";
import {
  useDeleteLibraryItem,
  useLibrary,
  useRestoreLibraryItem,
} from "./library-hooks";

const columnHelper = createColumnHelper<LibraryItem>();

const columns = [
  columnHelper.accessor("title", { header: "library.columns.title" }),
  columnHelper.accessor("status", { header: "library.columns.status" }),
  columnHelper.accessor("updatedAt", { header: "library.columns.updated" }),
];

const statusClass: Record<LibraryStatus, string> = {
  active: "border-success/30 bg-success/10 text-success",
  draft: "border-warning/30 bg-warning/10 text-warning",
  archived: "border-border bg-muted text-muted-foreground",
};

export function LibraryPage() {
  const { t } = useTranslation();
  const location = useLocation();
  const navigate = useNavigate();
  const { itemId } = useParams();
  const [search, setSearch] = useState("");
  const [status, setStatus] = useState<LibraryListInput["status"]>("all");
  const [sort, setSort] = useState<LibraryListInput["sort"]>("updated");
  const [direction, setDirection] = useState<LibraryListInput["direction"]>("desc");
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [creating, setCreating] = useState(false);
  const [dragging, setDragging] = useState(false);
  const deferredSearch = useDeferredValue(search);
  const list = useLibrary({ search: deferredSearch, status, sort, direction });
  const remove = useDeleteLibraryItem();
  const restore = useRestoreLibraryItem();
  const items = useMemo(() => list.data ?? [], [list.data]);
  const selected = items.find((item) => item.id === selectedId) ?? null;
  const table = useReactTable({ data: items, columns, getCoreRowModel: getCoreRowModel() });
  const scrollRef = useRef<HTMLDivElement>(null);
  const rowVirtualizer = useVirtualizer({
    count: items.length,
    getScrollElement: () => scrollRef.current,
    estimateSize: () => 47,
    overscan: 10,
  });

  useEffect(() => {
    if (itemId) {
      setSelectedId(itemId);
      setCreating(false);
    }
  }, [itemId]);

  useEffect(() => {
    if ((location.state as { newItem?: boolean } | null)?.newItem) {
      setCreating(true);
      setSelectedId(null);
      navigate(location.pathname, { replace: true, state: null });
    }
  }, [location.pathname, location.state, navigate]);

  const toggleSort = (next: LibraryListInput["sort"]) => {
    if (sort === next) {
      setDirection(direction === "asc" ? "desc" : "asc");
      return;
    }
    setSort(next);
    setDirection(next === "title" ? "asc" : "desc");
  };

  const deleteItem = async (item: LibraryItem) => {
    await remove.mutateAsync(item.id);
    if (selectedId === item.id) {
      setSelectedId(null);
      navigate("/library");
    }
    toast(t("library.deleted"), {
      action: {
        label: t("library.undo"),
        onClick: () => void restore.mutateAsync(item.id),
      },
    });
  };

  const importFile = async (file: File) => {
    const path = window.desktopApi.files.pathForFile(file);
    if (!path) {
      toast.error(t("common.couldNotRead"));
      return;
    }
    try {
      const result = await invoke(window.desktopApi.library.importPath(path));
      toast.success(t("common.imported", { count: result.imported }));
    } catch (error) {
      toast.error(error instanceof DesktopApiError ? error.message : t("common.error"));
    }
  };

  return (
    <div
      className="relative flex h-full min-h-0 flex-col"
      onDragLeave={() => setDragging(false)}
      onDragOver={(event) => { event.preventDefault(); setDragging(true); }}
      onDrop={(event) => {
        event.preventDefault();
        setDragging(false);
        const file = event.dataTransfer.files[0];
        if (file) void importFile(file);
      }}
    >
      {dragging ? (
        <div className="absolute inset-4 z-20 grid place-items-center rounded-xl border-2 border-dashed border-primary bg-background/90 text-sm font-medium">
          {t("common.importDrop")}
        </div>
      ) : null}
      <div className="flex flex-wrap items-center gap-2 border-b border-border px-4 py-3">
        <div className="relative min-w-56 flex-1">
          <Search className="pointer-events-none absolute left-2.5 top-1/2 size-4 -translate-y-1/2 text-muted-foreground" />
          <Input
            aria-label={t("common.search")}
            className="pl-8"
            onChange={(event) => setSearch(event.target.value)}
            placeholder={t("library.searchPlaceholder")}
            value={search}
          />
        </div>
        <Select onValueChange={(value) => setStatus(value as LibraryListInput["status"])} value={status}>
          <SelectTrigger aria-label={t("library.status")} className="w-36">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">{t("common.all")}</SelectItem>
            <SelectItem value="active">{t("library.active")}</SelectItem>
            <SelectItem value="draft">{t("library.draft")}</SelectItem>
            <SelectItem value="archived">{t("library.archived")}</SelectItem>
          </SelectContent>
        </Select>
        <Button onClick={() => { setCreating(true); setSelectedId(null); }}>
          <Plus className="size-4" />
          {t("library.newItem")}
        </Button>
      </div>

      <ResizablePanelGroup className="min-h-0 flex-1" orientation="horizontal">
        <ResizablePanel defaultSize={62} minSize={34}>
          <div className="flex h-full min-h-0 flex-col">
            <div
              className="grid shrink-0 grid-cols-[minmax(0,1fr)_110px_140px] gap-3 border-b border-border px-4 py-2 text-xs font-medium text-muted-foreground"
              role="row"
            >
              {table.getHeaderGroups()[0]?.headers.map((header) => {
                const key = header.column.id === "title" ? "title" : header.column.id === "status" ? "status" : "updated";
                const active = (key === "title" && sort === "title") || (key === "updated" && sort === "updated");
                return (
                  <button
                    className="flex items-center gap-1 text-left"
                    key={header.id}
                    onClick={() => toggleSort(key as LibraryListInput["sort"])}
                    type="button"
                  >
                    {typeof header.column.columnDef.header === "string"
                      ? t(header.column.columnDef.header)
                      : flexRender(header.column.columnDef.header, header.getContext())}
                    {active ? direction === "asc" ? <ArrowUp className="size-3" /> : <ArrowDown className="size-3" /> : null}
                  </button>
                );
              })}
            </div>
            <div className="scrollbar-thin min-h-0 flex-1 overflow-auto" ref={scrollRef} role="rowgroup">
              {list.isLoading ? (
                <div className="grid gap-2 p-4">
                  {Array.from({ length: 5 }, (_, index) => (
                    <div className="h-9 animate-pulse rounded-md bg-muted" key={index} />
                  ))}
                </div>
              ) : items.length === 0 ? (
                <Empty className="border-0">
                  <EmptyHeader>
                    <EmptyMedia variant="icon">▤</EmptyMedia>
                    <EmptyTitle>{t("library.emptyTitle")}</EmptyTitle>
                    <EmptyDescription>{t("library.emptyDescription")}</EmptyDescription>
                  </EmptyHeader>
                </Empty>
              ) : (
                <div className="relative w-full" style={{ height: rowVirtualizer.getTotalSize() }}>
                  {rowVirtualizer.getVirtualItems().map((virtualRow) => {
                    const item = items[virtualRow.index];
                    return (
                      <div
                        aria-selected={selectedId === item.id}
                        className={cn(
                          "grid cursor-default grid-cols-[minmax(0,1fr)_110px_140px] items-center gap-3 border-b border-border/70 px-4 text-sm outline-none hover:bg-muted/60 data-[selected=true]:bg-accent",
                          selectedId === item.id && "bg-accent",
                        )}
                        data-testid={`library-row-${item.id}`}
                        key={item.id}
                        onClick={() => { setSelectedId(item.id); setCreating(false); navigate(`/library/${item.id}`); }}
                        onKeyDown={(event) => {
                          if (event.key === "Enter" || event.key === " ") {
                            event.preventDefault();
                            setSelectedId(item.id);
                            setCreating(false);
                            navigate(`/library/${item.id}`);
                          }
                        }}
                        role="row"
                        style={{ position: "absolute", top: 0, transform: `translateY(${virtualRow.start}px)`, width: "100%", height: `${virtualRow.size}px` }}
                        tabIndex={0}
                      >
                        <div className="min-w-0" role="cell">
                          <p className="truncate font-medium">{item.title}</p>
                          <p className="truncate text-xs text-muted-foreground">{item.description || t("common.noDescription")}</p>
                          {/* smokeforge:anchor sync-row-status */}
                        </div>
                        <div role="cell"><StatusBadge status={item.status} /></div>
                        <div className="text-xs text-muted-foreground" role="cell">
                          {formatDistanceToNow(new Date(item.updatedAt), { addSuffix: true })}
                        </div>
                      </div>
                    );
                  })}
                </div>
              )}
            </div>
          </div>
        </ResizablePanel>
        <ResizableHandle withHandle />
        <ResizablePanel defaultSize={38} minSize={26}>
          <div className="flex h-full min-h-0 flex-col overflow-auto p-5">
            {creating || selected ? (
              <>
                {/* smokeforge:anchor sync-conflict */}
                <div className="mb-4 flex items-start justify-between gap-3">
                  <div>
                    <h2 className="text-sm font-semibold">{creating ? t("library.newItem") : t("library.editItem")}</h2>
                    {selected ? <p className="mt-1 text-xs text-muted-foreground">{t("common.updated")} {formatDistanceToNow(new Date(selected.updatedAt), { addSuffix: true })}</p> : null}
                  </div>
                  {selected ? (
                    <div className="flex items-center gap-1">
                      <Button aria-label={t("common.openInNewWindow")} onClick={() => void invoke(window.desktopApi.windows.openLibraryItem(selected.id))} size="icon" variant="ghost">
                        <ExternalLink className="size-4" />
                      </Button>
                      <Button aria-label={t("common.delete")} onClick={() => void deleteItem(selected)} size="icon" variant="ghost">
                        <Trash2 className="size-4 text-destructive" />
                      </Button>
                    </div>
                  ) : null}
                </div>
                <LibraryForm item={selected} onDone={(item) => { setCreating(false); setSelectedId(item.id); }} />
                {selected ? (
                  <>
                    <Separator className="my-5" />
                    <div className="grid gap-2 text-xs text-muted-foreground">
                      <p>{t("common.created")} {formatDistanceToNow(new Date(selected.createdAt), { addSuffix: true })}</p>
                      <p>{t("common.id")} {selected.id.slice(0, 8)}</p>
                    </div>
                  </>
                ) : null}
              </>
            ) : (
              <Empty className="border-0">
                <EmptyHeader>
                  <EmptyMedia variant="icon">⌁</EmptyMedia>
                  <EmptyTitle>{t("library.selectTitle")}</EmptyTitle>
                  <EmptyDescription>{t("library.selectDescription")}</EmptyDescription>
                </EmptyHeader>
              </Empty>
            )}
          </div>
        </ResizablePanel>
      </ResizablePanelGroup>
    </div>
  );
}

function StatusBadge({ status }: { status: LibraryStatus }) {
  const { t } = useTranslation();

  return (
    <Badge className={cn("capitalize", statusClass[status])} variant="outline">
      {t(`library.${status}`)}
    </Badge>
  );
}
