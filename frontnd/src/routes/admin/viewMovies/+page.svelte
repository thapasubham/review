<script>
	import { onMount } from 'svelte';
	import { writable } from 'svelte/store';
	import Dashboard from '../dashboard.svelte';

	let movies = [];

	async function fetchMovies() {
		try {
			const response = await fetch('http://localhost:3000/admin/view/movie'); // Update with your API endpoint
			if (!response.ok) {
				throw new Error('Failed to fetch movies');
			}
			movies = await response.json();
		} catch (error) {
			console.error(error);
		}
	}

	onMount(() => {
		fetchMovies();
	});
</script>

<Dashboard />
<div class="movie-container">
	<h1>Movie List</h1>

	{#if movies.length > 0}
		<table>
			<thead>
				<tr>
					<th>Movie Name</th>
					<th>Genre</th>
					<th>Released</th>
					<th>Total Reviews</th>
				</tr>
			</thead>
			<tbody>
				{#each movies as movie, count}
					<tr>
						<td><a href="/admin/review/?m_id={movie.movie_id}">{movie.movie_name}</a></td>
						<td>{movie.genre}</td>
						<td>{movie.released}</td>
						<td>{movie.total_reviews}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	{:else}
		<p>No movies found.</p>
	{/if}
</div>

<style>
	@import '../body.css';

	.movie-container {
		width: 80%;
		margin: 0 auto;
		text-align: center;
	}

	h1 {
		margin-bottom: 20px;
	}

	table {
		width: 100%;
		border-collapse: collapse;
		margin: 20px 0;
		font-size: 18px;
		text-align: left;
	}

	th,
	td {
		padding: 12px 15px;
		border: 1px solid #0d543b;
	}

	th {
		background-color: #0a595f;
		font-weight: bold;
		color: white;
	}

	tr:nth-child(even) {
		background-color: #0e4165;
	}
	a {
		text-decoration-line: none;
		color: white;
	}
	a:hover {
		color: rgb(4, 71, 26);
	}
	tr:hover {
		background-color: #036484;
	}
</style>
