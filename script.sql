insert into bookmarks (url) values ("reddit.com");
insert into bookmarks (url) values ("new.ycombinator.com");
insert into bookmarks (url) values ("facebook.com");
insert into bookmarks (url) values ("google.com");
insert into bookmarks (url) values ("duckduckgo.com");
insert into bookmarks (url) values ("1337x.to");

insert into tags (tag) values ("social media");
insert into tags (tag) values ("personal");
insert into tags (tag) values ("work");
insert into tags (tag) values ("programming");

insert into bookmarks_tags (bookmark, tag) values (1,1);
insert into bookmarks_tags (bookmark, tag) values (1,2);
insert into bookmarks_tags (bookmark, tag) values (2,1);
insert into bookmarks_tags (bookmark, tag) values (2,4);
insert into bookmarks_tags (bookmark, tag) values (3,1);
insert into bookmarks_tags (bookmark, tag) values (4,2);
