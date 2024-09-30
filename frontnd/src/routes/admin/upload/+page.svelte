<script>
	import { onMount } from 'svelte';

	// @ts-nocheck

	import Dashboard from '../dashboard.svelte';
	import { goto } from '$app/navigation';

	let name = '';
	let released = '';
	let bio = '';
	let genre = 'Comedy';
	let file = File;

	onMount(() => {
		const token = localStorage.getItem('token');
		if (!token) {
			// If token exists, redirect to home
			goto('/admin');
		}
	});

	async function handleSubmit(event) {
		event.preventDefault();

		const formData = new FormData();
		formData.append('name', name);
		formData.append('released', released);
		formData.append('bio', bio);
		formData.append('genre', genre);
		formData.append('image', file[0]);

		const response = await fetch('http://127.0.0.1:3000/admin/upload', {
			method: 'POST',
			body: formData,
			headers: {
				Accept: 'application/json'
			}
		});
		const data = await response.json();
		alert(data.message);
	}
</script>

<Dashboard />
<div class="form-box">
	<form on:submit|preventDefault={handleSubmit} enctype="multipart/form-data">
		<label>Movie Name</label>
		<input type="text" bind:value={name} required />

		<label>Released Year</label>
		<input type="text" bind:value={released} required />

		<label>About Movie</label>
		<textarea bind:value={bio} required></textarea>

		<label>Genre</label>
		<select bind:value={genre}>
			<option value="Comedy">Comedy</option>
			<option value="Action">Action</option>
			<option value="Crime">Crime</option>
			<option value="Thriller">Thriller</option>
			<option value="Sci-Fi">Sci-Fi</option>
		</select>

		<label>Upload Image</label>
		<input type="file" accept="image/png, image/jpeg" bind:files={file} required />

		<button type="submit">Submit</button>
	</form>
</div>

<style>
	@import '../body.css';

	.form-box {
		padding: 20px;
		border-radius: 5px;
		background-color: #040f25;
		box-shadow: 0 0 5px rgba(200, 255, 100, 0.21);
		width: 25%;
		margin: 10px auto;
	}
	h1 {
		text-align: center;
		margin-top: 20px;
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

	input[type='submit'] {
		background-color: #007bff;
		color: #fff;
		padding: 10px 20px;
		border: none;
		border-radius: 5px;
		cursor: pointer;
		font-size: 16px;
		margin-top: 10px;
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
	input[type='submit']:hover {
		background-color: #0056b3;
	}
</style>
