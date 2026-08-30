import type { AppStateData, Preset, Rule, WatchedFolder } from "./types";

export const BUNDLED_PRESET_IDS = {
  everyday: "11111111-1111-4111-8111-111111111101",
  media: "11111111-1111-4111-8111-111111111102",
  developer: "11111111-1111-4111-8111-111111111103",
  downloads: "11111111-1111-4111-8111-111111111104",
} as const;

const BUNDLED_IDS = new Set<string>(Object.values(BUNDLED_PRESET_IDS));
const MAX_PRESETS = 50;

function extensionRule(id: string, name: string, values: string[], subdirectory: string): Rule {
  return {
    id,
    name,
    enabled: true,
    matchAll: true,
    criteria: [{ kind: "extension", values }],
    action: { kind: "moveTo", subdirectory },
  };
}

export function bundledPresets(): Preset[] {
  return [
    {
      id: BUNDLED_PRESET_IDS.everyday,
      name: "Everyday tidy",
      description: "Sort common documents, images, archives, audio, and video into clear folders.",
      rules: [
        extensionRule("21111111-1111-4111-8111-111111111101", "Images", ["jpg", "jpeg", "png", "gif", "webp", "svg", "heic"], "Images"),
        extensionRule("21111111-1111-4111-8111-111111111102", "Documents", ["pdf", "doc", "docx", "txt", "md", "rtf", "odt", "xls", "xlsx", "ppt", "pptx"], "Documents"),
        extensionRule("21111111-1111-4111-8111-111111111103", "Archives", ["zip", "7z", "rar", "tar", "gz", "bz2", "xz"], "Archives"),
        extensionRule("21111111-1111-4111-8111-111111111104", "Audio", ["mp3", "wav", "flac", "m4a", "aac", "ogg"], "Audio"),
        extensionRule("21111111-1111-4111-8111-111111111105", "Video", ["mp4", "mkv", "mov", "webm", "avi", "m4v"], "Video"),
      ],
    },
    {
      id: BUNDLED_PRESET_IDS.media,
      name: "Media library",
      description: "Group image, audio, and video files below a single Media folder.",
      rules: [
        extensionRule("22111111-1111-4111-8111-111111111101", "Media images", ["jpg", "jpeg", "png", "gif", "webp", "svg", "heic", "avif"], "Media/Images"),
        extensionRule("22111111-1111-4111-8111-111111111102", "Media audio", ["mp3", "wav", "flac", "m4a", "aac", "ogg", "opus"], "Media/Audio"),
        extensionRule("22111111-1111-4111-8111-111111111103", "Media video", ["mp4", "mkv", "mov", "webm", "avi", "m4v"], "Media/Video"),
      ],
    },
    {
      id: BUNDLED_PRESET_IDS.developer,
      name: "Developer workspace",
      description: "Separate common source, data/configuration, and package files for project staging folders.",
      rules: [
        extensionRule("23111111-1111-4111-8111-111111111101", "Source code", ["rs", "ts", "tsx", "js", "jsx", "py", "java", "kt", "swift", "go", "php", "cs", "cpp", "c", "h", "hpp"], "Development/Source"),
        extensionRule("23111111-1111-4111-8111-111111111102", "Data and configuration", ["json", "yaml", "yml", "toml", "xml", "csv", "ini", "env"], "Development/Data"),
        extensionRule("23111111-1111-4111-8111-111111111103", "Packages and archives", ["zip", "7z", "tar", "gz", "tgz", "bz2", "xz"], "Development/Packages"),
      ],
    },
    {
      id: BUNDLED_PRESET_IDS.downloads,
      name: "Downloads cleanup",
      description: "Tidy common downloads into installers, archives, documents, and images.",
      rules: [
        extensionRule("24111111-1111-4111-8111-111111111101", "Installers", ["exe", "msi", "msix", "dmg", "pkg", "deb", "rpm", "appimage"], "Installers"),
        extensionRule("24111111-1111-4111-8111-111111111102", "Downloaded archives", ["zip", "7z", "rar", "tar", "gz", "bz2", "xz"], "Archives"),
        extensionRule("24111111-1111-4111-8111-111111111103", "Downloaded documents", ["pdf", "doc", "docx", "txt", "md", "rtf", "odt", "xls", "xlsx", "ppt", "pptx"], "Documents"),
        extensionRule("24111111-1111-4111-8111-111111111104", "Downloaded images", ["jpg", "jpeg", "png", "gif", "webp", "svg", "heic", "avif"], "Images"),
      ],
    },
  ];
}

export function isBundledPresetId(id: string | undefined): boolean {
  return Boolean(id && BUNDLED_IDS.has(id));
}

export interface BundledPresetUpgrade {
  state: AppStateData;
  changed: boolean;
  missingPresetCount: number;
}

export function upgradeBundledPresets(input: AppStateData): BundledPresetUpgrade {
  let changed = false;
  let presets = input.presets;
  let watchedFolders = input.watchedFolders;
  const catalog = bundledPresets();
  const everyday = catalog[0];

  if (!presets.some(preset => preset.id === everyday.id)) {
    const legacyIndex = presets.findIndex(preset => preset.name === everyday.name);
    if (legacyIndex >= 0) {
      const oldId = presets[legacyIndex].id;
      presets = presets.map((preset, index) => index === legacyIndex ? { ...preset, id: everyday.id } : preset);
      watchedFolders = remapWatchedPreset(watchedFolders, oldId, everyday.id);
      changed = true;
    }
  }

  for (const bundled of catalog) {
    if (presets.some(preset => preset.id === bundled.id) || presets.length >= MAX_PRESETS) continue;
    if (presets === input.presets) presets = [...presets];
    presets.push(bundled);
    changed = true;
  }

  const missingPresetCount = catalog.filter(bundled => !presets.some(preset => preset.id === bundled.id)).length;
  if (!changed) return { state: input, changed: false, missingPresetCount };
  return { state: { ...input, presets, watchedFolders }, changed: true, missingPresetCount };
}

function remapWatchedPreset(watches: WatchedFolder[], fromId: string, toId: string): WatchedFolder[] {
  if (!watches.some(watch => watch.presetId === fromId)) return watches;
  return watches.map(watch => watch.presetId === fromId ? { ...watch, presetId: toId } : watch);
}
