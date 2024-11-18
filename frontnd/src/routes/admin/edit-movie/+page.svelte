<script>
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import Dashboard from '../dashboard.svelte';

	let m_id = parseInt($page.url.searchParams.get('m_id'));
	let img = $page.url.searchParams.get('img');
	let name = '';
	let released = '';
	let bio = '';
	let genre = '';
	let img_name = '';

	async function handleEdit() {
		const response = await fetch('http://127.0.0.1:3000/admin/movie/edit', {
			method: 'PUT',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ movie_id: m_id, name, released, bio, genre })
		});

		const data = await response.json();
		if (data.success) {
			goto('/admin/home');
		} else {
			console.error('Edit failed:', data.message);
		}
	}

	onMount(async () => {
		//Fetch existing movie data here by ID
		const res = await fetch(`http://127.0.0.1:3000/admin/movie/${m_id}`);
		const movie = await res.json();
		// Populate the form fields with existing data
		if (movie) {
			name = movie.movie_name;
			released = movie.released;
			bio = movie.about_movie;
			genre = movie.genre;
			img_name = movie.img_name;
		}
	});
</script>

<Dashboard />

<div class="real">
	<h1>Edit Movie</h1>
	<div class="container">
		<img class="image" src="http://127.0.0.1:3000/image/{img}" alt={name} />
		<form on:submit|preventDefault={handleEdit}>
			<label for="name">Movie Name:</label>
			<input type="text" id="name" bind:value={name} required />

			<label for="released">Released:</label>
			<input type="text" id="released" bind:value={released} required />

			<label for="bio">Bio:</label>
			<textarea id="bio" bind:value={bio} required></textarea>

			<label>Genre</label>
			<select bind:value={genre}>
				<option value="Comedy">Comedy</option>
				<option value="Action">Action</option>
				<option value="Crime">Crime</option>
				<option value="Thriller">Thriller</option>
				<option value="Sci-Fi">Sci-Fi</option>
			</select>

			<button type="submit">Update Movie</button>
		</form>
	</div>
</div>

<style>
	@import '../body.css';

	img {
		max-width: 100%;
		max-height: 200px;
		padding: 10px;
		width: auto;
		height: auto;
		margin: 0 auto;
		display: block;
	}

	.real {
		margin-top: 10px;
		padding: 20px;
		border-radius: 5px;
		background-color: #040f25;
		box-shadow: 0 0 5px rgba(200, 255, 100, 0.21);
		width: 50%;
		margin: 10px auto;
	}
	h1 {
		text-align: center;
		margin-top: 20px;
	}
	select {
		width: 40%;
		padding: 10px;
		margin-top: 5px;
		border: 1px solid #ccc;
		border-radius: 5px;
		font-size: 16px;
	}
	form {
		max-width: 400px;
		margin: 0 auto;
		padding: 20px;
		border-radius: 5px;
	}

	label {
		display: block;
		font-weight: bold;
		margin-top: 10px;
		margin-bottom: 5px;
	}

	input[type='text'] {
		width: 98%;
		padding: 10px;
		margin-top: 5px;
		border: 1px solid #ccc;
		border-radius: 5px;
		font-size: 16px;
	}
	textarea {
		stroke: none;
		width: 98%;
		height: 100px;
		padding: 10px;
		margin-top: 5px;
		border: 1px solid #ccc;
		border-radius: 5px;
		font-size: 16px;
	}
	button[type='submit'] {
		background-color: #007bff;
		color: #fff;
		padding: 10px 20px;
		border: none;
		border-radius: 5px;
		cursor: pointer;
		font-size: 16px;
		margin-top: 10px;
	}

	button[type='submit']:hover {
		background-color: #0056b3;
	}
</style>
