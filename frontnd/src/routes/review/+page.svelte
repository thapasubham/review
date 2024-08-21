<script>
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import MovieDetails from '../movielist/MovieDetails.svelte';
	import { fetchMovieDetails } from './review';
	import { fetchReviews } from './review';
	import HeaderComp from '../HeaderComp.svelte';
	let loggedIn;

	//const token = cookies.get('token');
	let id = $page.url.searchParams.get('id');
	let movie = '';
	let reviews = '';
	let averageStar = 0.0;
	onMount(async () => {
		try {
			const data = await fetchMovieDetails(id);
			movie = data.movie || {};
			averageStar = parseFloat(data.star);

			reviews = await fetchReviews(id);
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
		<p class="year">Releases: {movie.released}</p>
		{#if averageStar == null}
			<p>No reviews yet</p>
		{:else}
			<p>Average rating: {averageStar}</p>
		{/if}
	</div>
</div>

<div class="comments"></div>

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
		width: 500px;
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
</style>
