create table bookmarks (
  id integer primary key autoincrement not null,
  url text not null,
  description text not null
);

create table tags (
  id integer primary key autoincrement not null,
  tag text not null
);

create table bookmarks_tags (
  id integer primary key autoincrement not null,
  bookmark_id integer not null,
  tag_id integer not null,
  foreign key(bookmark_id) references bookmarks(id),
  foreign key(tag_id) references tags(id)
);
