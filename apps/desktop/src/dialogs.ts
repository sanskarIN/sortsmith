import { open, save } from "@tauri-apps/plugin-dialog";

export async function chooseFolder(defaultPath?: string): Promise<string | null> {
  const selected = await open({
    directory: true,
    multiple: false,
    defaultPath: defaultPath?.trim() || undefined,
    title: "Choose a folder for SortSmith",
  });

  if (Array.isArray(selected)) {
    return selected[0] ?? null;
  }

  return selected;
}

export async function chooseSettingsImport(): Promise<string | null> {
  const selected = await open({
    directory: false,
    multiple: false,
    title: "Import SortSmith settings",
    filters: [{ name: "SortSmith settings", extensions: ["json"] }],
  });

  if (Array.isArray(selected)) {
    return selected[0] ?? null;
  }

  return selected;
}

export async function chooseSettingsExport(): Promise<string | null> {
  return save({
    title: "Export SortSmith settings",
    defaultPath: "sortsmith-settings.json",
    filters: [{ name: "SortSmith settings", extensions: ["json"] }],
  });
}
