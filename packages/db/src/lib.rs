#![allow(non_snake_case)]

use self::error::*;
use ::surrealdb::engine::local::{ Db, SpeeDb, TiKv };
use ::surrealdb::Surreal;

pub mod error;

pub struct Database {
	db: Surreal<Db>
}

impl Database {
	/// For local development, stores data written to the provided location on disk.
	/// Ideally not for production; for that use tikv connection
	#[inline]
	pub async fn connect_speedb(db_path: &str) -> Result<Self> {
		let db = Surreal::new::<SpeeDb>(db_path).await?;
		Ok(Self { db })
	}

	/// Connect to remote TiKV instance.
	#[inline]
	pub async fn connect_tikv(db_addr: &str) -> Result<Self> {
		let db = Surreal::new::<TiKv>(db_addr).await?;
		Ok(Self { db })
	}

	pub async fn user__create(&self, opts: opts::UserCreate<'_>) -> Result {
		self.db.query(include_str!("./queries/user__create.surrealql"))
			.bind(opts)
			.await?;
		Ok(())
	}



	/*
	we'd want essentially one db function per action we can do almost
	set_user_bio
	set_user_display_name
	create_story
	set_story_title
	publish_story
	set_story_cover (takes url? and cover loading colour)
	get_story_completion_status
	set_story_completion_status
	get_content_rating
	set_content_rating

	create_chapter
	set_chapter_title
	set_chapter_content
	get_chapter_views
	add_chapter_view
	set_chapter_authors_note_top
	set_chapter_authors_note_bottom
	delete_chapter

	create_bookshelf
	set_bookshelf_name
	set_bookshelf_icon
	set_bookshelf_colour

	add_story_author
	remove_story_author

	add_story_like
	remove_story_like

	add_story_dislike
	remove_story_dislike

	add_story_to_bookshelf
	remove_story_from_bookshelf

	add_story_sequel
	request_story_sequel_add

	follow_user

	comment_on_story

	mark_story_read

	get_global_ranking
	get_global_word_ranking

	 */
}

pub mod opts {
	use ::serde::Serialize;

	#[derive(Serialize)]
	pub struct UserCreate<'h> {
		pub email: &'h str,
		pub username: &'h str
	}
}
