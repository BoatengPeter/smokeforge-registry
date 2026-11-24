import { Command as CommandIcon } from "lucide-react";
import { useTranslation } from "react-i18next";

import {
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  CommandShortcut,
} from "@/components/ui/command";
import { appCommands, formatAccelerator, type CommandId } from "@shared/commands";

const isMac = navigator.userAgent.includes("Mac");

export function CommandPalette({
  open,
  onOpenChange,
  onRun,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onRun: (id: CommandId) => void;
}) {
  const { t } = useTranslation();

  return (
    <CommandDialog
      description={t("commands.placeholder")}
      onOpenChange={onOpenChange}
      open={open}
      title={t("commands.title")}
    >
      <CommandInput placeholder={t("commands.placeholder")} />
      <CommandList>
        <CommandEmpty>{t("commands.noResults")}</CommandEmpty>
        <CommandGroup heading={t("commands.title")}>
          {appCommands.map((command) => (
            <CommandItem
              key={command.id}
              onSelect={() => {
                onOpenChange(false);
                onRun(command.id);
              }}
              value={`${t(command.labelKey)} ${command.id}`}
            >
              <CommandIcon className="size-4 opacity-50" />
              {t(command.labelKey)}
              {command.accelerator ? <CommandShortcut>{formatAccelerator(command.accelerator, isMac)}</CommandShortcut> : null}
            </CommandItem>
          ))}
        </CommandGroup>
      </CommandList>
    </CommandDialog>
  );
}
