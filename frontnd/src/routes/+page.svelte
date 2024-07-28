<script>
	import { onMount } from 'svelte';
	import HeaderComp from './HeaderComp.svelte';
	let name = 'http://127.0.0.1:3000/image/Drive.jpeg';
</script>

<HeaderComp />

<body>
	<div class="welcome">
		<p>Welcome</p>
	</div>
	<div class="movie-list">
		<img src={name} alt="Drive" />
		<p><a href="/movielist/">View Movie</a></p>
	</div>

	<p>Review and Rate movies</p>
</body>

<style>
	@import '../body.css';
	@import '../style.css';

	.welcome {
		text-align: left;
	}

	.movie-list {
		text-align: center;
		margin-top: 50px;
	}

	.movie-list img {
		max-width: 100%;
		max-height: 400px;
		width: auto;
		height: auto;
		display: block;
		margin: 0 auto;
		border-radius: 10px;
		box-shadow: 0 0 10px rgba(0, 0, 0, 0.2);
	}

	.movie-list p {
		font-size: 18px;
		margin-top: 15px;
	}

	.additional-content p {
		font-size: 16px;
		line-height: 1.5;
		color: #666;
	}
	p {
		text-align: center;
		font-size: 20px;
	}
</style>
