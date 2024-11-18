<script>
	import { onMount } from 'svelte';
	import MovieDetails from './MovieDetails.svelte';
	import HeaderComp from '../HeaderComp.svelte';
	let movie = [];
	onMount(async () => {
		const cookieHeader = document.cookie;
		const cookies = new Map(cookieHeader.split('; ').map((cookie) => cookie.split('=')));
		const token = cookies.get('token');

		try {
			if (token) {
				const response = await fetch(`http://127.0.0.1:3000/review/movies/${token}`);
				movie = await response.json();
			} else {
				const response = await fetch('http://127.0.0.1:3000/movie/id');
				movie = await response.json();
			}
		} catch (error) {
			console.log(error);
		}
	});
</script>

<HeaderComp />

<div class="movies">
	{#each movie as movielist}
		<MovieDetails id={movielist.movie_id} />
	{/each}
</div>

<style>
	.movies {
		display: flex;
		flex-wrap: wrap;
		justify-content: space-around;
	}
</style>
