use actix_web::{actix::Handler, error, Error};
use chrono::Utc;
use db::article::{Article, ArticleId, ArticleList, ArticleNew, NewArticle};
use diesel::{self, sql_query, ExpressionMethods, QueryDsl, RunQueryDsl};
use model::db::ConnDsl;
use model::response::{ArticleListMsgs, ArticleMsgs, Msgs};

impl Handler<ArticleList> for ConnDsl {
    type Result = Result<ArticleListMsgs, Error>;

    fn handle(&mut self, article_list: ArticleList, _: &mut Self::Context) -> Self::Result {
        use share::schema::article::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let articles = article
            .load::<Article>(conn)
            .map_err(error::ErrorInternalServerError)?;
        Ok(ArticleListMsgs {
            status: 200,
            message: "article_list result Success.".to_string(),
            article_list: articles,
        })
    }
}
