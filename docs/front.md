# Home: `/`
## `no-auth` Login: `/login`
## `no-auth` Registration: `/signup`
## Team selector: `/teams`
- Create
- Select
## Profile: `/profile`
- View profile
- Edit profile
## Team creator: `/new`
## Team dashboard: `/<team>/`
- Config
- Security: Users, Roles
- Apps:
  - Lists
  - Marvin
  - Inventory
### Team configuration: `/<team>/config`
- Manage name, description, icon
- Delete team
### Team security: `/<team>/security`
- Add users
- Remove users
- Manage roles
# Apps: `/<team>/<app>`

## Product app: `/<team>/products`
- Add
- Edit
- Delete (Also from lists)
### Product creator: `/<team>/products/new`
- Name
- Description
- Icon
- Images
- Tags
### Product editor: `/<team>/products/<product>`

## List app: `/<team>/lists`
- Create
- Manage
### List: `/<team>/lists/<list>`
- Create category
- Manage categories
- Delete category
- Add product to category
- Manage product
- Remove product from category

## Tag app: `/<team>/tags`
- Create
- Manage
- Delete

## Ideas:
- Accessibility
- Offline mode
- Export features
- Budget manager
### Home:
- Astro
### Editors:
- AI Icon suggestion
- AI Description
