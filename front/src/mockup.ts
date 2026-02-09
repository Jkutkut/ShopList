import type { Product, Team, User } from "./types";

export const IMAGES = {
  "1:1": "/assets/blank-1.1.svg",
  "2:1": "/assets/blank-2.1.svg",
}

export const TEXT = {
  "1": "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
  "2": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
  "3": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.",
  "4": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
  "5": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
}

export const USER: User & { email: string } = {
  id: "user-a",
  name: "Jkutkut",
  email: "email@email.com",
  image: IMAGES["1:1"],
  created_at: new Date().toISOString(),
  updated_at: new Date().toISOString(),
};

export const PRODUCTS: Product[] = [
  {
    id: "product-a",
    name: "Product A",
    description: TEXT["1"],
    image: IMAGES["1:1"],
    created_at: new Date().toISOString(),
    created_by: USER.id,
    updated_at: new Date().toISOString(),
    updated_by: USER.id
  },
  {
    id: "product-b",
    name: "Product B",
    description: TEXT["2"],
    image: IMAGES["1:1"],
    created_at: new Date().toISOString(),
    created_by: USER.id,
    updated_at: new Date().toISOString(),
    updated_by: USER.id
  },
  {
    id: "product-c",
    name: "Product C",
    description: TEXT["3"],
    image: IMAGES["1:1"],
    created_at: new Date().toISOString(),
    created_by: USER.id,
    updated_at: new Date().toISOString(),
    updated_by: USER.id
  },
  {
    id: "product-d",
    name: "Product D",
    description: TEXT["1"],
    image: IMAGES["1:1"],
    created_at: new Date().toISOString(),
    created_by: USER.id,
    updated_at: new Date().toISOString(),
    updated_by: USER.id
  },
  {
    id: "product-e",
    name: "Product E",
    description: TEXT["2"],
    image: IMAGES["1:1"],
    created_at: new Date().toISOString(),
    created_by: USER.id,
    updated_at: new Date().toISOString(),
    updated_by: USER.id
  },
  {
    id: "product-f",
    name: "Product F",
    description: TEXT["3"],
    image: undefined,
    created_at: new Date().toISOString(),
    created_by: USER.id,
    updated_at: new Date().toISOString(),
    updated_by: USER.id
  }
];

export const LISTS = [
  {
    id: "list-a",
    name: "List A",
    description: TEXT["1"],
    image: IMAGES["1:1"],
    created_at: new Date().toISOString(),
    created_by: USER.id,
    updatedAt: new Date().toISOString(),
    updated_by: USER.id
  },
  {
    id: "list-b",
    name: "List B",
    description: TEXT["2"],
    image: IMAGES["1:1"],
    created_at: new Date().toISOString(),
    created_by: USER.id,
    updatedAt: new Date().toISOString(),
    updated_by: USER.id
  },
  {
    id: "list-c",
    name: "List C",
    description: TEXT["3"],
    image: IMAGES["1:1"],
    created_at: new Date().toISOString(),
    created_by: USER.id,
    updatedAt: new Date().toISOString(),
    updated_by: USER.id
  }
];

export const CATEGORIES = [
  {
    id: "category-a",
    name: "Category A",
    listId: "list-a",
    index: 1,
    createdAt: new Date().toISOString(),
    createdBy: USER.id,
    updatedAt: new Date().toISOString(),
    updatedBy: USER.id
  },
  {
    id: "category-b",
    name: "Category B",
    listId: "list-a",
    index: 2,
    createdAt: new Date().toISOString(),
    createdBy: USER.id,
    updatedAt: new Date().toISOString(),
    updatedBy: USER.id
  },
  {
    id: "category-c",
    name: "Category C",
    listId: "list-b",
    index: 3,
    createdAt: new Date().toISOString(),
    createdBy: USER.id,
    updatedAt: new Date().toISOString(),
    updatedBy: USER.id
  },
];

export const LIST_PRODUCTS = {
  "list-a": [
    ...PRODUCTS.slice(0, 3).map((p, idx) => ({
      id: "list-product-" + idx,
      listId: "list-a",
      categoryId: "category-a",
      productId: p.id,
      index: idx + 1,
      amount: "12",
      unit: "L",
      createdAt: new Date().toISOString(),
      createdBy: USER.id,
      updatedAt: new Date().toISOString(),
      updatedBy: USER.id
    })),
    {
      id: "list-product-3",
      listId: "list-a",
      categoryId: undefined,
      productId: PRODUCTS[3].id,
      index: 3 + 1,
      createdAt: new Date().toISOString(),
      createdBy: USER.id,
      updatedAt: new Date().toISOString(),
      updatedBy: USER.id
    },
    ...PRODUCTS.splice(4, -1).map((p, idx) => ({
      id: "list-product-" + (idx + 4),
      listId: "list-a",
      categoryId: "category-b",
      productId: p.id,
      index: idx + 3 + 1,
      createdAt: new Date().toISOString(),
      createdBy: USER.id,
      updatedAt: new Date().toISOString(),
      updatedBy: USER.id
    })),
  ],
  "list-b": [],
  "list-c": [],
};

export const TEAMS: (Team & { products: typeof PRODUCTS, lists: typeof LISTS })[] = [
  {
    id: "team-a",
    name: "Team A",
    description: TEXT["2"],
    image: IMAGES["1:1"],
    products: PRODUCTS,
    lists: LISTS,
    created_at: new Date().toISOString(),
    created_by: USER.id,
    updated_at: new Date().toISOString(),
    updated_by: USER.id
  },
  {
    id: "team-b",
    name: "Team B",
    description: TEXT["3"],
    products: PRODUCTS,
    lists: LISTS,
    created_at: new Date().toISOString(),
    created_by: USER.id,
    updated_at: new Date().toISOString(),
    updated_by: USER.id
  },
  {
    id: "team-c",
    name: "Team C",
    description: TEXT["1"],
    image: IMAGES["1:1"],
    products: PRODUCTS,
    lists: LISTS,
    created_at: new Date().toISOString(),
    created_by: USER.id,
    updated_at: new Date().toISOString(),
    updated_by: USER.id
  },
  {
    id: "team-d",
    name: "Team D",
    description: TEXT["2"],
    products: PRODUCTS,
    lists: LISTS,
    created_at: new Date().toISOString(),
    created_by: USER.id,
    updated_at: new Date().toISOString(),
    updated_by: USER.id
  },
  {
    id: "team-e",
    name: "Team E",
    description: TEXT["3"],
    image: IMAGES["1:1"],
    products: PRODUCTS,
    lists: LISTS,
    created_at: new Date().toISOString(),
    created_by: USER.id,
    updated_at: new Date().toISOString(),
    updated_by: USER.id
  },
  {
    id: "team-f",
    name: "Team F",
    description: TEXT["1"],
    products: PRODUCTS,
    lists: LISTS,
    created_at: new Date().toISOString(),
    created_by: USER.id,
    updated_at: new Date().toISOString(),
    updated_by: USER.id
  }
];

export const ROLES = ["admin", "member"];

export const TEAM_ROLES = TEAMS.map((team, idx) => ({
  role: (idx !== 1 ? "admin" : "member"),
  team: team
}));

export const ALL_USERS = [
  USER,
  {
    id: "user-b",
    name: "marvin01",
    email: "marvin01@email.com",
    image: IMAGES["1:1"],
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
  },
  {
    id: "user-c",
    name: "marvin02",
    email: "marvin02@email.com",
    image: IMAGES["1:1"],
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
  },
  {
    id: "user-d",
    name: "marvin03",
    email: "marvin03@email.com",
    image: IMAGES["1:1"],
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
  }
];

export const ALL_TEAM_ROLES = TEAMS.map((team, idx) => ({
  key: team.id,
  value: ALL_USERS.map((user, userIdx) => ({
    role: (
      USER.id === user.id ?
      (TEAM_ROLES.find(role => role.team.id === team.id)?.role || undefined) :
      (idx === userIdx) ? "admin" : "member"
    ),
    user
  }))
}))
.reduce<Record<string, any>>((acc, team) => {
  acc[team.key] = team.value;
  return acc;
}, {});


export const OPTIONS: { [key: string]: { name: string; url: string }[] } = {};
OPTIONS['logout'] = [
  { name: 'Logout', url: '/logout' },
];
OPTIONS['profile'] = [
  { name: 'Teams', url: '/teams' },
  ...OPTIONS['logout'],
];
OPTIONS['profile-edit'] = [
  { name: 'Back', url: '/profile' },
  ...OPTIONS['profile'],
];
OPTIONS['teams'] = [
  { name: 'Profile', url: '/profile' },
  ...OPTIONS['logout'],
];
OPTIONS['team'] = [
  { name: 'Teams', url: '/teams' },
  { name: 'Profile', url: '/profile' },
  ...OPTIONS['logout'],
];
OPTIONS['lists'] = OPTIONS['teams'];
OPTIONS['list'] = [
  { name: 'Config list', url: './config' },
  { name: 'Profile', url: '/profile' },
  ...OPTIONS['logout'],
];
OPTIONS['products'] = OPTIONS['teams'];
OPTIONS['new-team'] = [
  { name: 'Teams', url: '/teams' },
  ...OPTIONS['logout'],
];
OPTIONS['config-team'] = [
  ...OPTIONS['logout'],
];
OPTIONS['security-team'] = OPTIONS['config-team'];

export const APPS = [
  { name: 'Shopping Lists', url: './lists', enabled: true },
  { name: 'Marvin', url: './marvin', enabled: false },
  { name: 'Products', url: './products', enabled: true },
  { name: 'Tags', url: './tags', enabled: false },
  { name: 'Security - Users', url: './security', enabled: true },
  { name: 'Team config', url: './config', enabled: true },
];

export const ACTION = (action: string) => {
  return (...args: any[]) => {
    if (args.length > 0) {
      if (args[0].preventDefault) {
        args[0].preventDefault();
      }
    }
    console.log(action, ...args);
  };
};
