export type Role = 'bidder' | 'seller' | 'admin' | 'moderator'

export type Permission =
  | 'bid:create'
  | 'auction:create'
  | 'auction:edit'
  | 'auction:approve'
  | 'user:ban'