import { defineMiddleware } from "astro/middleware";

const notAuthRootPages: (string | RegExp)[] = [
  "/",
  "/login",
  "/register",
  /^\/d{3}\/?$/
];

export const onRequest = defineMiddleware((context, next) => {
  const jwt = context.cookies.get("jwt");
  const isAuthenticated = Boolean(jwt);

  const currentPath = context.url.pathname;

  if (isAuthenticated && currentPath === "/") {
    return context.redirect("/teams", 302);
  }
  const isAuthPage = !notAuthRootPages.some((page) => {
    if (page instanceof RegExp) {
      return page.test(currentPath);
    }
    return currentPath === page;
  });
  if (!isAuthenticated && isAuthPage) {
    return context.redirect("/login", 302);
  }

  return next();
});
