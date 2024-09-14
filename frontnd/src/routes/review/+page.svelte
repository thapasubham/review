<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import MovieDetails from '../movielist/MovieDetails.svelte';
	import { fetchMovieDetails } from './review';
	import { fetchReviews } from './review';
	import HeaderComp from '../HeaderComp.svelte';
	let loggedIn;

	let id = $page.url.searchParams.get('id');
	let movie = '';
	let reviews = '';
	let averageStar = 0.0;
	let userReview = '';
	const token = '';
	onMount(async () => {
		const cookieHeader = document.cookie;
		// @ts-ignore
		const cookies = new Map(cookieHeader.split('; ').map((cookie) => cookie.split('=')));
		const token = cookies.get('token');

		try {
			const data = await fetchMovieDetails(id);
			movie = data.movie || {};
			averageStar = parseFloat(data.star);

			reviews = await fetchReviews(id);

			userReview = await reviews.find((reviews) => reviews.reviewed_by == token);

			if (userReview) {
				console.log(userReview.reviewed_by);
				console.log(token);
			} else {
				console.log('You havent reviewed');
			}
		} catch (error) {
			('');
			console.error('Error in component:', error);
		}
	});
</script>

<HeaderComp />
<div class="outer_box">
	<div class="movie_box">
		<img
			class="image"
			src={`http://127.0.0.1:3000/image/${movie.img_name}`}
			alt={movie.movie_name}
		/>
		<p class="movie_name">{movie.movie_name}</p>
		<p class="genre">{movie.genre}</p>
		<p class="year">Releases: {movie.released}</p>
		{#if averageStar == null}
			<p>No reviews yet</p>
		{:else}
			<p>Average rating: {averageStar}</p>
		{/if}
	</div>
</div>
{#if reviews.length == 0}
	<p>No reviews Yet</p>
{:else}
	{#if userReview}
		<div class="comments-box">
			<div class="each-comment">
				<p style="font-weight: bold;">Your Review</p>
				<p>{userReview.review_msg}</p>
				<p>Rating: {userReview.star}</p>
			</div>
		</div>
	{:else}
		<p>pp</p>
	{/if}
	<div class="comments-box">
		{#each reviews as review}
			{#if review.reviewed_by != userReview.reviewed_by}
				<div class="each-comment">
					<p style="font-weight: bold;">{review.firstname}</p>

					<p>{review.review_msg}</p>
					<p>
						{review.star}
					</p>
				</div>
			{/if}
		{/each}
	</div>{/if}

<style>
	@import '../../body.css';
	.outer_box {
		display: flex;
		flex-wrap: wrap;
		height: 600px;
		justify-content: space-around;
		padding: 20px;
	}
	h3 {
		text-align: center;
	}

	.movie_box {
		width: 600px;
		height: auto;
		padding: 10px;
		background-color: #040f25;
		border-radius: 5px;
		box-shadow: 0 0 10px rgba(255, 0255, 255, 0.2);
		text-align: center;
		align-items: center;
		justify-content: center;
	}

	.movie_name {
		font-weight: bold;
	}

	.year {
		color: #afafaf;
	}

	.image {
		height: 400px;
		max-width: 400px;
		border-radius: 5px;
		margin-top: 5px;
	}
	.link {
		margin-top: 5px;
	}
	.msg {
		width: 80%;
		color: white;
		margin: 0 auto;
		text-align: center;
		padding: 20px;
	}

	.comments-box {
		height: auto;
		width: 40%;
		margin: 0 auto;

		padding: 20px;
		border-radius: 5px;
	}
	.each-comment {
		width: auto;

		text-align: left;
		padding: 15px;
		box-shadow: 0 0 10px rgb(0, 0, 0);
	}
	select {
		width: 10%;
		padding: 10px;
		margin-top: 5px;
		border: 1px solid #ccc;
		border-radius: 5px;
		font-size: 16px;
	}
	.delete:hover {
		color: red;
	}
</style>
