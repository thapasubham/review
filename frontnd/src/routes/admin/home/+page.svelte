<script>
	import { onMount } from 'svelte';
	import Dashboard from '../dashboard.svelte';
	import { goto } from '$app/navigation';
	import MovieDetails from './MovieDetails.svelte';

	let movie = [];
	onMount(async () => {
		const token = localStorage.getItem('token');
		if (!token) {
			// If token exists, redirect to home
			goto('/admin');
		}
		try {
			const response = await fetch('http://127.0.0.1:3000/admin/movie');
			movie = await response.json();
			//console.log(movie);
		} catch (error) {
			console.log(error);
		}
	});
</script>

<Dashboard />

<div class="movies">
	{#each movie as movielist}
		<MovieDetails
			id={movielist.movie_id}
			name={movielist.movie_name}
			img_name={movielist.img_name}
			released={movielist.released}
			genre={movielist.genre}
			bio={movielist.about_movie}
		/>
	{/each}
</div>

<style>
	@import '../body.css';
	.movies {
		display: flex;
		flex-wrap: wrap;
		justify-content: space-around;
	}
</style>
