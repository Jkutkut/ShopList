import { defineMiddleware } from "astro/middleware";

const authRootPages = [
  "/new",
  "/profile",
  "/teams",
  /^\/[\w\d]{8}-[\w\d]{4}-[\w\d]{4}-[\w\d]{4}-[\w\d]{12}\//, // TODO use UUID_V4_REGEX
];

export const onRequest = defineMiddleware((context, next) => {
  const jwt = context.cookies.get("jwt");
  const isAuthenticated = Boolean(jwt);

  const currentPath = context.url.pathname;

  if (isAuthenticated && currentPath === "/") {
    return context.redirect("/teams", 302);
  }
  const isAuthPage = authRootPages.some((page) => {
    if (page instanceof RegExp) {
      return page.test(currentPath);
    }
    return currentPath.startsWith(page);
  });
  if (!isAuthenticated && isAuthPage) {
    return context.redirect("/login", 302);
  }

  return next();
});
