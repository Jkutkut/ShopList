import { defineMiddleware } from "astro/middleware";

const authRootPages = [
  "/new",
  "/profile",
  "/teams"
];

export const onRequest = defineMiddleware((context, next) => {
  const jwt = context.cookies.get("jwt");
  const isAuthenticated = Boolean(jwt);

  const currentPath = context.url.pathname;

  if (isAuthenticated && currentPath === "/") {
    return context.redirect("/teams", 302);
  }
  const isAuthPage = authRootPages.some((page) => currentPath.startsWith(page));
  if (!isAuthenticated && isAuthPage) {
    return context.redirect("/login", 302);
  }

  return next();
});
