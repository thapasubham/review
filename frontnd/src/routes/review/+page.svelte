<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { fetchMovieDetails, fetchReviews } from './review';
	import HeaderComp from '../HeaderComp.svelte';

	let id = $page.url.searchParams.get('id');
	let movie = '';
	let reviews = [];
	let averageStar = 0.0;
	let userReview = null;
	let msg = '';
	let edit = false;
	let token = '';

	onMount(async () => {
		const cookieHeader = document.cookie;
		const cookies = new Map(cookieHeader.split('; ').map((cookie) => cookie.split('=')));
		token = cookies.get('token');

		try {
			const data = await fetchMovieDetails(id);
			movie = data.movie || {};
			averageStar = parseFloat(data.star);

			reviews = await fetchReviews(id);
			userReview = reviews.find((review) => review.reviewed_by == token);
		} catch (error) {
			console.error('Error in component:', error);
		}
	});

	let star = '';
	function edited() {
		edit = true;

		msg = userReview.review_msg;

		star = userReview.star;
		console.log(star);
	}

	async function deleteReview() {
		try {
			const response = await fetch(`http://127.0.0.1:3000/review/delete/${userReview.review_id}`, {
				method: 'DELETE'
			});

			const result = await response.json();
			if (response.ok) {
				window.location.reload();
				alert(result); // Show success message
			} else {
				throw new Error(result);
			}
		} catch (error) {
			console.error('Error deleting review:', error);
		}
	}

	async function handleSubmit(event) {
		event.preventDefault(); // Prevent default form submission

		const formData = new FormData(event.target); // Collect form data
		const reviewedById = parseInt(token);

		try {
			const payload = JSON.stringify({
				reviewed_by: reviewedById,
				movie_id: parseInt(id),
				review_msg: msg,
				star: parseInt(formData.get('star'))
			});

			let response;
			if (edit === false) {
				response = await fetch('http://127.0.0.1:3000/review/add', {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json'
					},
					body: payload
				});
			} else {
				response = await fetch(`http://127.0.0.1:3000/review/edit/`, {
					method: 'PUT', // Use PUT for updates
					headers: {
						'Content-Type': 'application/json'
					},
					body: payload
				});
				edit = false;
			}

			// Check if the response is not empty
			const resultText = await response.text();
			if (resultText) {
				const result = JSON.parse(resultText); // Parse response only if not empty
				if (!response.ok) {
					throw new Error('Network response was not ok');
				}
				alert(result);
				window.location.reload(); // Use the message from the backend
				console.log(result);
			} else {
				console.log('No response data received');
			}
		} catch (error) {
			console.error('Error submitting review:', error);
		}
	}
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
		<p class="genre">{movie.genre}</p>
		<p class="year">Released: {movie.released}</p>
		{#if averageStar == null}
			<p>No reviews yet</p>
		{:else}
			<p>Average rating: {averageStar}</p>
		{/if}
	</div>
</div>

<!-- Show review form if userReview doesn't exist or user clicked Edit -->
{#if token}
	{#if !userReview || edit}
		<div class="comments-box">
			<div class="each-comment">
				<form on:submit={handleSubmit}>
					<label>Add your review</label>
					<input type="text" name="review_msg" bind:value={msg} required />
					<input type="hidden" name="movie_id" value={movie.movie_id} />
					<label>Rating</label>
					<select name="star" value={star} required>
						<option value="1">1</option>
						<option value="2">2</option>
						<option value="3">3</option>
						<option value="4">4</option>
						<option value="5">5</option>
					</select>
					<input type="submit" value="Submit" />
				</form>
			</div>
		</div>
	{/if}

	<!-- Show user review if exists and not in edit mode -->
	{#if userReview && !edit}
		<div class="comments-box">
			<div class="each-comment">
				<p style="font-weight: bold;">Your Review</p>
				<p>{userReview.review_msg}</p>
				<p>Rating: {userReview.star}</p>
				<button on:click={edited}>Edit</button>

				<button on:click={deleteReview}>Delete</button>
			</div>
		</div>
	{/if}
{:else}
	<p style="text-align: center;"><a href="/login">Login</a> To add review</p>
{/if}
<!-- Show other reviews -->
{#if reviews.length == 0}
	<p style="text-align: center">No reviews yet</p>
{:else}
	<div class="comments-box">
		{#each reviews as review}
			{#if review.reviewed_by !== userReview?.reviewed_by}
				<div class="each-comment">
					<p style="font-weight: bold;">{review.firstname}</p>
					<p>{review.review_msg}</p>
					<p>Rating: {review.star}</p>
				</div>
			{/if}
		{/each}
	</div>
{/if}

<style>
	@import '../../body.css';
	@import '../../style.css';
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
