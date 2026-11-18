import { closestCenter, DndContext, KeyboardSensor, PointerSensor, useSensor, useSensors, type DragEndEvent } from "@dnd-kit/core";
import { arrayMove, SortableContext, sortableKeyboardCoordinates, useSortable } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { formatDistanceToNow } from "date-fns";
import type { ReactNode } from "react";
import { ArrowUpRight, Database, Layers, Sparkles } from "lucide-react";
import { useTranslation } from "react-i18next";
import { Link } from "react-router";
import { Area, AreaChart, CartesianGrid, XAxis } from "recharts";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { ChartContainer, ChartTooltip, ChartTooltipContent } from "@/components/ui/chart";
import { Empty, EmptyDescription, EmptyHeader, EmptyTitle } from "@/components/ui/empty";
import { useLibraryStats } from "@renderer/features/library/library-hooks";
import { useAppSettings } from "@renderer/lib/app-settings";

const chartData = [
  { day: "Mon", items: 4 },
  { day: "Tue", items: 7 },
  { day: "Wed", items: 5 },
  { day: "Thu", items: 11 },
  { day: "Fri", items: 9 },
  { day: "Sat", items: 14 },
  { day: "Sun", items: 12 },
];

const chartConfig = {
  items: { label: "Local records", color: "var(--chart-1)" },
};

export function DashboardPage() {
  const { t } = useTranslation();
  const stats = useLibraryStats();
  const recent = stats.data?.recent ?? [];
  const { settings, update } = useAppSettings();
  const order = settings?.dashboardOrder ?? ["active", "total", "archived"];
  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, { coordinateGetter: sortableKeyboardCoordinates }),
  );
  const cards = {
    active: { icon: <Layers className="size-4" />, label: t("dashboard.active"), value: stats.data?.active ?? "—" },
    total: { icon: <Database className="size-4" />, label: t("dashboard.total"), value: stats.data?.total ?? "—" },
    archived: { icon: <Sparkles className="size-4" />, label: t("dashboard.archived"), value: stats.data?.archived ?? "—" },
  } as const;

  const onDragEnd = (event: DragEndEvent) => {
    const { active, over } = event;
    if (!over || active.id === over.id) return;
    const next = arrayMove(
      order,
      order.findIndex((id) => id === String(active.id)),
      order.findIndex((id) => id === String(over.id)),
    );
    void update({ dashboardOrder: next });
  };

  return (
    <div className="mx-auto grid w-full max-w-7xl gap-6 p-6">
      <DndContext collisionDetection={closestCenter} onDragEnd={onDragEnd} sensors={sensors}>
        <SortableContext items={order}>
          <section className="grid gap-3 sm:grid-cols-3">
            {order.map((id) => (
              <SortableStatCard key={id} id={id} {...cards[id as keyof typeof cards]} />
            ))}
          </section>
        </SortableContext>
      </DndContext>

      <section className="grid gap-4 lg:grid-cols-[1.3fr_1fr]">
        <Card>
          <CardHeader className="flex-row items-center justify-between">
            <CardTitle>{t("dashboard.chart")}</CardTitle>
            <Badge variant="secondary">{t("common.sevenDays")}</Badge>
          </CardHeader>
          <CardContent>
            <ChartContainer className="h-64 w-full" config={chartConfig}>
              <AreaChart data={chartData} margin={{ left: 0, right: 8, top: 8 }}>
                <defs>
                  <linearGradient id="fill-items" x1="0" x2="0" y1="0" y2="1">
                    <stop offset="5%" stopColor="var(--color-items)" stopOpacity={0.5} />
                    <stop offset="95%" stopColor="var(--color-items)" stopOpacity={0.04} />
                  </linearGradient>
                </defs>
                <CartesianGrid vertical={false} stroke="var(--border)" />
                <XAxis axisLine={false} dataKey="day" tickLine={false} tickMargin={10} />
                <ChartTooltip content={<ChartTooltipContent />} />
                <Area dataKey="items" fill="url(#fill-items)" stroke="var(--color-items)" strokeWidth={2} type="monotone" />
              </AreaChart>
            </ChartContainer>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex-row items-center justify-between">
            <CardTitle>{t("dashboard.recent")}</CardTitle>
            <Button asChild size="sm" variant="ghost">
              <Link to="/library">
                {t("dashboard.open")}
                <ArrowUpRight className="size-3.5" />
              </Link>
            </Button>
          </CardHeader>
          <CardContent>
            {recent.length === 0 ? (
              <Empty className="border-0 p-2">
                <EmptyHeader>
                  <EmptyTitle>{t("common.empty")}</EmptyTitle>
                  <EmptyDescription>{t("dashboard.empty")}</EmptyDescription>
                </EmptyHeader>
              </Empty>
            ) : (
              <ul className="grid gap-1">
                {recent.map((item) => (
                  <li key={item.id}>
                    <Link className="flex items-center justify-between gap-3 rounded-md px-2 py-2.5 text-sm hover:bg-muted" to={`/library/${item.id}`}>
                      <span className="min-w-0">
                        <span className="block truncate font-medium">{item.title}</span>
                        <span className="block text-xs text-muted-foreground">
                          {formatDistanceToNow(new Date(item.updatedAt), { addSuffix: true })}
                        </span>
                      </span>
                      <Badge className="capitalize" variant="outline">{t(`library.${item.status}`)}</Badge>
                    </Link>
                  </li>
                ))}
              </ul>
            )}
          </CardContent>
        </Card>
      </section>
    </div>
  );
}

function SortableStatCard({ id, icon, label, value }: { id: string; icon: ReactNode; label: string; value: number | string }) {
  const { attributes, listeners, setNodeRef, transform, transition } = useSortable({ id });

  return (
    <Card
      className="cursor-grab active:cursor-grabbing"
      data-testid={`stat-${id}`}
      ref={setNodeRef}
      style={{ transform: CSS.Transform.toString(transform), transition }}
      {...attributes}
      {...listeners}
    >
      <CardHeader className="flex-row items-center gap-2">
        <span className="grid size-8 place-items-center rounded-md bg-accent text-accent-foreground">{icon}</span>
        <CardTitle className="text-xs font-medium text-muted-foreground">{label}</CardTitle>
      </CardHeader>
      <CardContent>
        <p className="text-3xl font-semibold tracking-tight">{value}</p>
      </CardContent>
    </Card>
  );
}
