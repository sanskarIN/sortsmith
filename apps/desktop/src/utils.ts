export function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  const units = ["KB", "MB", "GB", "TB"];
  let value = bytes / 1024;
  let index = 0;
  while (value >= 1024 && index < units.length - 1) { value /= 1024; index += 1; }
  return `${value.toFixed(value >= 10 ? 1 : 2)} ${units[index]}`;
}

export function shortPath(path: string, max = 56): string {
  if (path.length <= max) return path;
  const keep = Math.max(8, Math.floor((max - 1) / 2));
  return `${path.slice(0, keep)}…${path.slice(-keep)}`;
}

export function dueInMinutes(lastRunAt: string | null | undefined, interval: number): boolean {
  if (!lastRunAt) return true;
  return Date.now() - new Date(lastRunAt).getTime() >= interval * 60_000;
}
