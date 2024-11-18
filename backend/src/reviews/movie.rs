use axum::{
    extract::{Extension, Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{mysql::MySqlPool, query, query_as, types::BigDecimal};
use std::collections::HashSet;
use tower::util::Optional;
#[derive(Serialize, Deserialize)]
struct Movie {
    movie_id: i32,
    movie_name: String,
    img_name: String,
    about_movie: String,
    genre: String,
    released: i32,
}

#[derive(Serialize, Deserialize)]
struct Review {
    review_id: i32,
    review_msg: String,
    star: i32,
    reviewed_by: i32,
    firstname: String,
}
pub async fn get_reviews(
    State(pool): State<MySqlPool>,
    Path(movie_id): Path<i32>,
) -> impl IntoResponse {
    let review = query_as!(
        Review,
        r#"Select review.review_id, review.review_msg, 
        review.star, review.reviewed_by, members.firstname from 
        review  INNER JOIN members on review.reviewed_by = m_id where movie_id=?"#,
        movie_id
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to load data");

    Json(review)
}

pub async fn get_details(
    State(pool): State<MySqlPool>,
    Path(movie_id): Path<i32>,
) -> impl IntoResponse {
    let details = query_as!(
        Movie,
        "Select movie_id,  movie_name, img_name, about_movie,genre, released from movie where movie_id =? ",
        movie_id
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to resolve");

    let avg_result = sqlx::query!(
        "SELECT AVG(star) AS average_star FROM review WHERE movie_id = ?",
        movie_id
    )
    .fetch_one(&pool)
    .await;

    let average_star_str = match avg_result.ok().and_then(|r| r.average_star) {
        Some(avg) => avg.to_string(),
        None => "0.0".to_string(),
    };

    let respond = json!({
        "movie": details,
        "star": average_star_str,
    });

    Json(respond)
}

#[derive(Serialize, Deserialize)]
struct Movie_ID {
    movie_id: i32,
}

pub async fn movies(State(pool): State<MySqlPool>) -> impl IntoResponse {
    let result = query_as!(Movie_ID, "SELECT movie_id FROM movie ")
        .fetch_all(&pool)
        .await
        .expect("Failed to get the movie details");
    Json(result)
}

#[derive(Clone, Serialize)]
pub struct MovieDetails {
    movie_id: i32,
    movie_name: String,
    about_movie: String,
    img_name: String,
    genre: String,
    released: i32,
}

#[derive(Clone)]
pub struct MovieVector {
    movie: MovieDetails,
    genre_vector: Vec<f32>,
}

pub async fn get_movies(State(pool): State<MySqlPool>, Path(u_id): Path<i16>) -> impl IntoResponse {
    let all_movies: Vec<MovieDetails> = query_as!(
        MovieDetails,
        "SELECT movie_id, movie_name, about_movie, img_name, genre, released FROM movie;"
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch all movies");

    let reviewed_movies: Vec<MovieDetails> = query_as!(
        MovieDetails,
        "SELECT movie.movie_id, movie.movie_name, movie.about_movie, movie.img_name, movie.genre, movie.released
         FROM movie
         JOIN review ON movie.movie_id = review.movie_id
         WHERE review.reviewed_by = ?;",
        u_id
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch reviewed movies");

    if reviewed_movies.is_empty() {
        return Json(all_movies); // No reviewed movies
    }

    //genres across all movies
    let mut genre_set: HashSet<String> = HashSet::new();
    for movie in &all_movies {
        let genres: Vec<&str> = movie.genre.split(',').map(|s| s.trim()).collect();
        for genre in genres {
            genre_set.insert(genre.to_string());
        }
    }
    let genre_list: Vec<String> = genre_set.into_iter().collect();

    //itterators to convert the movie into a vector
    let all_movie_vectors: Vec<MovieVector> = all_movies
        .into_iter()
        .map(|movie| {
            let genres: Vec<&str> = movie.genre.split(',').map(|s| s.trim()).collect();
            let mut genre_vector = vec![0.0; genre_list.len()];
            for (i, genre) in genre_list.iter().enumerate() {
                if genres.contains(&genre.as_str()) {
                    genre_vector[i] = 1.0;
                }
            }
            MovieVector {
                movie,
                genre_vector,
            }
        })
        .collect();

    //same as above for reviewed movie
    let reviewed_movie_vectors: Vec<MovieVector> = reviewed_movies
        .into_iter()
        .map(|movie| {
            let genres: Vec<&str> = movie.genre.split(',').map(|s| s.trim()).collect();
            let mut genre_vector = vec![0.0; genre_list.len()];
            for (i, genre) in genre_list.iter().enumerate() {
                if genres.contains(&genre.as_str()) {
                    genre_vector[i] = 1.0;
                }
            }
            MovieVector {
                movie,
                genre_vector,
            }
        })
        .collect();

    //cosine similarity between each movie and the reviewed movies
    let mut similarities = vec![];
    for movie_vector in &all_movie_vectors {
        let mut similarity_sum = 0.0;
        for reviewed_vector in &reviewed_movie_vectors {
            similarity_sum +=
                cosine_similarity(&movie_vector.genre_vector, &reviewed_vector.genre_vector);
        }
        let avg_similarity = similarity_sum / reviewed_movie_vectors.len() as f32;
        similarities.push((movie_vector.clone(), avg_similarity));
    }

    similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let sorted_movies: Vec<MovieDetails> =
        similarities.into_iter().map(|(mv, _)| mv.movie).collect();
    Json(sorted_movies)
}

fn cosine_similarity(vec1: &Vec<f32>, vec2: &Vec<f32>) -> f32 {
    let dot_product: f32 = vec1.iter().zip(vec2.iter()).map(|(a, b)| a * b).sum();
    let magnitude_vec1: f32 = vec1.iter().map(|a| a.powi(2)).sum::<f32>().sqrt();
    let magnitude_vec2: f32 = vec2.iter().map(|b| b.powi(2)).sum::<f32>().sqrt();

    if magnitude_vec1 == 0.0 || magnitude_vec2 == 0.0 {
        return 0.0;
    }

    dot_product / (magnitude_vec1 * magnitude_vec2)
}
