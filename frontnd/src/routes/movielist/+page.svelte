<script>
	import { onMount } from 'svelte';
	import MovieDetails from './MovieDetails.svelte';
	let movie = [];
	onMount(async () => {
		const response = await fetch('http://127.0.0.1:3000/get_movies');
		movie = await response.json();
	});
</script>

<div class="movies">
	{#each movie as movielist}
		<MovieDetails id={movielist.movie_id} />
	{/each}
</div>

<style>
	.MovieDetails {
		display: flex;
	}
</style>
