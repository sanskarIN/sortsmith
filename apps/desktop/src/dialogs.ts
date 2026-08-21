import { open } from "@tauri-apps/plugin-dialog";

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
