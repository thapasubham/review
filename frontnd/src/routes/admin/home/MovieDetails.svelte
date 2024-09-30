<script>
	export let id;
	export let name;
	export let img_name;
	export let released;
	export let genre;
	export let bio;
</script>

<div class="movie_box">
	<img class="image" src={`http://127.0.0.1:3000/image/${img_name}`} alt={name} />
	<h2>{name}</h2>

	<p>Released: {released}</p>
	<p>Genre: {genre}</p>
	<a href="/admin/edit-movie/?m_id={id}&img={img_name}">Edit</a>
</div>

<style>
	@import '../body.css';
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
