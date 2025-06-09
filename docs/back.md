## Back:
### Server:
- Has teams
- Has Plugins
### Teams:
Instances of the application running on the same server.
- Completely independent of each other.
### Auth:
- Global authentication: for all teams
- Role for each user per team:
  - Admin: edit, add and remove users
  - Regular user: edit
  - Read only user: view
- Registration of users
- Login / logout of users
### List:
A topic defined shopping collection of products.
- Products listed are independent between lists.
- Contains multiple views for the list.

### Product:
- Available on a team level.
- Can be generic or particular
- An image can be added.
- A set of identifiers can be added.
  - Barcodes
- A particular product can have a generic related product.
- A generic product can have a particular related products.
- A product can have a single category.
- A product can have multiple tags.

### Category:
- Available on a team level.

### Tag:
- Available on a team level.

### View:
- Available of a List level.
- Collection of view filters

### View Filter:
- Set of categories and tags

