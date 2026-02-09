export const asUrl = (url: string, Astro: any) => {
  if (url.startsWith("./")) {
    url = Astro.url.pathname + url.slice(1);
  }
  return url;
};

export const previousPageUrl = (Astro: any) =>
  Astro.request.url.split('/').slice(0, -1).join('/') || '/';

export const UUID_V4_REGEX = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;
export const VALID_TEAM_REGEX = /^[a-z][a-zA-Z0-9_\-]{1,48}[a-zA-Z0-9]$/;

export const asDate = (date: Date | string | number) => (typeof date === "object") ? date : new Date(date);
