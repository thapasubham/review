<script>
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import Dashboard from '../dashboard.svelte';
	import { movieDetails, userReviews } from './review';

	/** @type {import('./$types').PageData} */
	export let data;
	let averageStar;
	let reviews = '';
	let movie = '';
	let m_id = $page.url.searchParams.get('m_id');
	onMount(async () => {
		try {
			const data = await movieDetails(m_id);
			movie = data.movie || {};
			averageStar = parseFloat(data.star);

			reviews = await userReviews(m_id);
			console.log(reviews);
		} catch (error) {
			console.error('Error in component:', error);
		}
	});
</script>

<Dashboard />
<div class="outer_box">
	<div class="movie_box">
		<p class="movie_name">{movie.movie_name}</p>
		<img
			class="image"
			src={`http://127.0.0.1:3000/image/${movie.img_name}`}
			alt={movie.movie_name}
		/>

		<p class="genre">{movie.genre}</p>
		<p class="year">Released: {movie.released}</p>
		<p>{movie.about_movie}</p>
		{#if averageStar == null}
			<p>No reviews yet</p>
		{:else}
			<p>Average rating: {averageStar}</p>
		{/if}
		<a href="/admin/edit-movie/?m_id={m_id}&img={movie.img_name}">Edit</a>
	</div>
</div>

{#if reviews.length == 0}
	<p class="no_rev" style="text-align: center;">No reviews yet</p>
{:else}
	<div class="comments-box">
		{#each reviews as review}
			<div class="each-comment">
				<p style="font-weight: bold;">{review.firstname}</p>
				<p>{review.review_msg}</p>
				<p>Rating: {review.star}</p>
			</div>
		{/each}
	</div>
{/if}

<style>
	@import '../body.css';
	@import '../../../style.css';
	.outer_box {
		display: flex;
		flex-wrap: wrap;
		height: auto;
		height: 600px;
		justify-content: space-around;
		padding: 20px;
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
	.no_rev {
		padding: 50px;
	}
	.link {
		margin-top: 5px;
	}
	h3 {
		text-align: center;
	}
	.movie_name {
		font-size: 20px;
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
		margin-top: 50px;
		padding: 20px;
		border-radius: 5px;
	}

	button {
		background-color: #007bff;
		color: #fff;
		padding: 5px 10px;
		border: none;
		border-radius: 5px;
		cursor: pointer;
		font-size: 16px;
		margin-top: 5px;
	}

	button:hover {
		background-color: #0056b3;
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
