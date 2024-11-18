<div class="admin-dashboard">
	<header>
		<h1>Admin Dashboard</h1>
		<nav>
			<ul>
				<li><a href="/admin/upload">Add Movie</a></li>
				<li><a href="/admin/viewUsers">View Users</a></li>
				<li><a href="/admin/viewMovies">Movie</a></li>
				<li><a href="/admin/home">Home</a></li>
				<li><a href="/admin/logout">Logout</a></li>
			</ul>
		</nav>
	</header>
</div>

<style>
	.admin-dashboard {
		align-items: center;
		text-align: center;
		margin: auto;
		width: 50%;
	}
	ul {
		width: 50%;
		justify-content: center;
		margin: auto;
		list-style-type: none;
		display: flex;
		padding: 5px;
		border-radius: 5px;
		box-shadow: 0 0 5px rgba(200, 255, 100, 0.21);
	}

	li {
		display: inline;
		margin-right: 10px;
	}

	a {
		text-align: right;
		text-decoration: none;
		color: #fff;
		font-weight: bold;
	}

	a:hover {
		color: #0577fa;
	}
</style>
