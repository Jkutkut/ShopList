export const IMAGES = {
  "1:1": "/assets/blank.svg",
}

export const TEXT = {
  "1": "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
  "2": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
  "3": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.",
  "4": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
  "5": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
}

export const USER = {
  id: "user-a",
  name: "Jkutkut",
  email: "email@email.com",
  image: IMAGES["1:1"],
  createdAt: new Date().toISOString(),
  updatedAt: new Date().toISOString(),
};

export const PRODUCTS = [
  {
    id: "product-a",
    name: "Product A",
    description: TEXT["1"],
    image: IMAGES["1:1"],
    createdAt: new Date().toISOString(),
    createdBy: USER.id,
    updatedAt: new Date().toISOString(),
    updatedBy: USER.id
  },
  {
    id: "product-b",
    name: "Product B",
    description: TEXT["2"],
    image: IMAGES["1:1"],
    createdAt: new Date().toISOString(),
    createdBy: USER.id,
    updatedAt: new Date().toISOString(),
    updatedBy: USER.id
  },
  {
    id: "product-c",
    name: "Product C",
    description: TEXT["3"],
    image: IMAGES["1:1"],
    createdAt: new Date().toISOString(),
    createdBy: USER.id,
    updatedAt: new Date().toISOString(),
    updatedBy: USER.id
  },
  {
    id: "product-d",
    name: "Product D",
    description: TEXT["1"],
    image: IMAGES["1:1"],
    createdAt: new Date().toISOString(),
    createdBy: USER.id,
    updatedAt: new Date().toISOString(),
    updatedBy: USER.id
  },
  {
    id: "product-e",
    name: "Product E",
    description: TEXT["2"],
    image: IMAGES["1:1"],
    createdAt: new Date().toISOString(),
    createdBy: USER.id,
    updatedAt: new Date().toISOString(),
    updatedBy: USER.id
  },
  {
    id: "product-f",
    name: "Product F",
    description: TEXT["3"],
    image: IMAGES["1:1"],
    createdAt: new Date().toISOString(),
    createdBy: USER.id,
    updatedAt: new Date().toISOString(),
    updatedBy: USER.id
  }
];

export const LISTS = [
  {
    id: "list-a",
    name: "List A",
    description: TEXT["1"],
    image: IMAGES["1:1"],
    createdAt: new Date().toISOString(),
    createdBy: USER.id,
    updatedAt: new Date().toISOString(),
    updatedBy: USER.id
  },
  {
    id: "list-b",
    name: "List B",
    description: TEXT["2"],
    image: IMAGES["1:1"],
    createdAt: new Date().toISOString(),
    createdBy: USER.id,
    updatedAt: new Date().toISOString(),
    updatedBy: USER.id
  },
  {
    id: "list-c",
    name: "List C",
    description: TEXT["3"],
    image: IMAGES["1:1"],
    createdAt: new Date().toISOString(),
    createdBy: USER.id,
    updatedAt: new Date().toISOString(),
    updatedBy: USER.id
  }
];

export const LIST_PRODUCTS = {
  "list-a": PRODUCTS,
  "list-b": PRODUCTS.slice(0, 3),
  "list-c": PRODUCTS.slice(3, 6),
};

export const TEAMS = [
  {
    id: "team-a",
    name: "Team A",
    description: TEXT["2"],
    image: IMAGES["1:1"],
    products: PRODUCTS,
    lists: LISTS,
  },
  {
    id: "team-b",
    name: "Team B",
    description: TEXT["3"],
    products: PRODUCTS,
    lists: LISTS,
  },
  {
    id: "team-c",
    name: "Team C",
    description: TEXT["1"],
    image: IMAGES["1:1"],
    products: PRODUCTS,
    lists: LISTS,
  },
  {
    id: "team-d",
    name: "Team D",
    description: TEXT["2"],
    products: PRODUCTS,
    lists: LISTS,
  },
  {
    id: "team-e",
    name: "Team E",
    description: TEXT["3"],
    image: IMAGES["1:1"],
    products: PRODUCTS,
    lists: LISTS,
  },
  {
    id: "team-f",
    name: "Team F",
    description: TEXT["1"],
    products: PRODUCTS,
    lists: LISTS,
  }
];

export const TEAM_ROLES = TEAMS.map((team, idx) => ({
  role: (idx === 1 ? "admin" : "member"),
  team: team
}));


export const OPTIONS: { [key: string]: { name: string; url: string }[] } = {};
OPTIONS['logout'] = [
  { name: 'Logout', url: '' },
];
OPTIONS['profile'] = [
  { name: 'Teams', url: '/teams' },
  ...OPTIONS['logout'],
];
OPTIONS['teams'] = [
  { name: 'Profile', url: '/profile' },
  ...OPTIONS['logout'],
];
OPTIONS['team'] = [
  { name: 'Config team', url: './config' },
  { name: 'Profile', url: '/profile' },
  ...OPTIONS['logout'],
];

export const APPS = [
  { name: 'Shopping Lists', url: './lists', enabled: false },
  { name: 'Marvin', url: './marvin', enabled: false },
  { name: 'Products', url: './products', enabled: false },
  { name: 'Tags', url: './tags', enabled: false },
  { name: 'Config', url: './config', enabled: false },
  { name: 'Security', url: './security', enabled: false },
];
