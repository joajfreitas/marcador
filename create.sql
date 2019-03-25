create table bookmarks (
	identifier INTEGER PRIMARY KEY, 
	url TEXT, 
	description TEXT);

create table tags (
	identifier INTEGER PRIMARY KEY, 
	tag TEXT);
    
create table bookmarks_tags (
	identifier INTEGER PRIMARY KEY,
	bookmark REFERENCES bookmarks(identifier), 
	tag REFERENCES tags(identifier));

