<script>
	// @ts-nocheck

	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import MovieDetails from '../movielist/MovieDetails.svelte';
	import { fetchMovieDetails } from './review';
	import { fetchReviews } from './review';
	let id = $page.url.searchParams.get('id');
	let movie = '';
	let reviews = '';
	let averageStar = 0.0;
	console.log('ID from URL:', id);
	onMount(async () => {
		try {
			const data = await fetchMovieDetails(id);
			movie = data.movie || {};
			averageStar = parseFloat(data.star);

			reviews = await fetchReviews(id);
		} catch (error) {
			console.error('Error in component:', error);
		}
	});
</script>

<div class="outer_box">
	<div class="movie_box">
		<img
			class="image"
			src={`http://127.0.0.1:3000/image/${movie.img_name}`}
			alt={movie.movie_name}
		/>
		<p class="movie_name">{movie.movie_name}</p>
		<p class="year">Releases: {movie.released}</p>
		{#if averageStar == null}
			<p>No reviews yet</p>
		{:else}
			<p>Average rating: {averageStar}</p>
		{/if}
	</div>
</div>

<div>
	{#each reviews as review}
		{review.firstname}
		{review.review_msg}
		{review.star}
		<br />
	{/each}
</div>

<style>
	@import '../../body.css';
	.outer_box {
		display: flex;
		flex-wrap: wrap;
		justify-content: space-around;
		padding: 20px;
	}
	h3 {
		text-align: center;
	}

	.movie_box {
		width: 20%;
		height: 50%;
		margin: 20px 0;
		padding: 15px;
		background-color: #040f25;
		border-radius: 5px;
		box-shadow: 0 0 10px rgba(255, 0255, 255, 0.2);
		transition: transform 0.3s;
		text-align: center;
	}

	.movie_name {
		font-weight: bold;
	}

	.year {
		color: #afafaf;
	}

	.image {
		max-width: 100%;
		max-height: 200px;
		border-radius: 5px;
		margin-top: 5px;
	}
	.link {
		margin-top: 5px;
	}
</style>
