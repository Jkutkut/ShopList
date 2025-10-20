export const asUrl = (url: string, Astro: any) => {
  if (url.startsWith("./")) {
    url = Astro.url.pathname + url.slice(1);
  }
  return url;
};

export const previousPageUrl = (Astro: any) =>
  Astro.request.url.split('/').slice(0, -1).join('/') || '/';
