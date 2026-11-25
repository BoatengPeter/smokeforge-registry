import { Plus } from "lucide-react";
import { useTranslation } from "react-i18next";

import { Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList, CommandShortcut } from "@/components/ui/command";
import { Dialog, DialogClose, DialogContent, DialogDescription, DialogTitle } from "@/components/ui/dialog";
import { appCommands, formatAccelerator, type CommandDefinition, type CommandId } from "@shared/commands";

const isMac = navigator.userAgent.includes("Mac");

/** Pages are the `nav:*` commands; everything else is an action ("+" row). */
const pages = appCommands.filter((command) => command.id.startsWith("nav:"));
const actions = appCommands.filter((command) => !command.id.startsWith("nav:") && command.id !== "app:palette");

/**
 * "Go to…" (sidebar search, Cmd/Ctrl+K): every page and action from
 * `appCommands`, in Herdr's proportions: 640 px wide, 40 px from the top, a
 * title with an Esc button, an accent-outlined search field, and 32 px rows.
 * Choosing a row runs the same command as its shortcut.
 */
export function GoTo({
  open,
  onOpenChange,
  onRun,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onRun: (id: CommandId) => void;
}) {
  const { t } = useTranslation();
  const run = (id: CommandId) => {
    onOpenChange(false);
    onRun(id);
  };

  return (
    <Dialog onOpenChange={onOpenChange} open={open}>
      <DialogContent
        className="top-10 grid translate-y-0 gap-2.5 overflow-hidden bg-popover p-[18px] sm:max-w-[640px] [&_[data-slot=command-input-wrapper]]:h-9 [&_[data-slot=command-input-wrapper]]:rounded-lg [&_[data-slot=command-input-wrapper]]:border [&_[data-slot=command-input-wrapper]]:border-primary [&_[data-slot=command-input-wrapper]]:bg-background"
        showCloseButton={false}
      >
        <div className="flex items-center">
          <DialogTitle className="flex-1 text-[15px] font-semibold">{t("goTo.title")}</DialogTitle>
          <DialogClose className="rounded-md px-1.5 py-0.5 text-[11px] text-muted-foreground hover:bg-accent hover:text-foreground">
            {t("goTo.escape")}
          </DialogClose>
        </div>
        <DialogDescription className="sr-only">{t("goTo.placeholder")}</DialogDescription>
        <Command className="bg-transparent">
          <CommandInput placeholder={t("goTo.placeholder")} />
          <CommandList className="mt-1 max-h-80">
            <CommandEmpty>{t("commands.noResults")}</CommandEmpty>
            <CommandGroup heading={t("goTo.pages")}>
              {pages.map((command) => (
                <GoToRow command={command} key={command.id} onRun={run} />
              ))}
            </CommandGroup>
            <CommandGroup heading={t("goTo.actions")}>
              {actions.map((command) => (
                <GoToRow command={command} key={command.id} onRun={run} plus />
              ))}
            </CommandGroup>
          </CommandList>
        </Command>
      </DialogContent>
    </Dialog>
  );
}

function GoToRow({ command, onRun, plus = false }: { command: CommandDefinition; onRun: (id: CommandId) => void; plus?: boolean }) {
  const { t } = useTranslation();
  const label = t(command.labelKey);

  return (
    <CommandItem className="h-8 rounded-[7px]" onSelect={() => onRun(command.id)} value={`${label} ${command.id}`}>
      {plus ? <Plus className="size-3.5 text-muted-foreground" /> : null}
      <span className={plus ? "text-muted-foreground" : undefined}>{label}</span>
      {command.accelerator ? <CommandShortcut>{formatAccelerator(command.accelerator, isMac)}</CommandShortcut> : null}
    </CommandItem>
  );
}
