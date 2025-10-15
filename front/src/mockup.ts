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

export const TEAMS = [
  {
    id: "team-a",
    name: "Team A",
    description: TEXT["2"],
    image: IMAGES["1:1"]
  },
  {
    id: "team-b",
    name: "Team B",
    description: TEXT["3"],
  },
  {
    id: "team-c",
    name: "Team C",
    description: TEXT["1"],
    image: IMAGES["1:1"]
  },
  {
    id: "team-d",
    name: "Team D",
    description: TEXT["2"],
  },
  {
    id: "team-e",
    name: "Team E",
    description: TEXT["3"],
    image: IMAGES["1:1"]
  },
  {
    id: "team-f",
    name: "Team F",
    description: TEXT["1"],
  }
];

export const TEAM_ROLES = TEAMS.map((team, idx) => ({
  role: (idx === 1 ? "admin" : "member"),
  team: team
}));

export const USER = {
  id: "user-a",
  name: "Jkutkut",
  email: "email@email.com",
  image: IMAGES["1:1"],
  createdAt: new Date().toISOString(),
  updatedAt: new Date().toISOString(),
};



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
