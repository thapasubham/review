<script>
	// @ts-nocheck

	import { onMount } from 'svelte';
	import { writable } from 'svelte/store';

	export let id;

	let movie = '';
	let averageStar = 0;

	onMount(async () => {
		try {
			const response = await fetch(`http://127.0.0.1:3000/review/${id}`);
			if (!response.ok) {
				throw new Error(`HTTP error! status: ${response.status}`);
			}
			const data = await response.json();
			console.log('Fetched data:', data); // Debugging statement
			movie = data.movie;
			averageStar = parseFloat(data.star); // Convert string to number
			console.log('Movie data:', movie); // Debugging statement
			console.log('Average star:', averageStar); // Debugging statement
		} catch (error) {
			console.error('Error fetching movie details:', error);
		}
	});
</script>

<div class="movie_box">
	<img class="image" src={`http://127.0.0.1:3000/image/${movie.img_name}`} alt={movie.movie_name} />
	<p class="movie_name">{movie.movie_name}</p>
	<p class="year">Releases: {movie.released}</p>
	{#if averageStar == null}
		<p>No reviews yet</p>
		<a href={`/movie/${movie.movie_id}`}>Add one</a>
	{:else}
		<p>Average rating: {averageStar}</p>
		<a href={`/review/?id=${movie.movie_id}`}>Check Reviews</a>
	{/if}
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

	.movie_box:hover {
		transform: scale(1.02);
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
	a {
		text-align: right;
		text-decoration: none;
		color: #fff;
		font-weight: bold;
	}
	a:hover {
		color: rgb(18, 2, 195);
	}
	.link {
		margin-top: 5px;
	}
</style>
