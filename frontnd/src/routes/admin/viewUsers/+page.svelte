<script>
	import { onMount } from 'svelte';
	import Dashboard from '../dashboard.svelte';

	let users = [];

	// Function to fetch users from the backend
	async function fetchUsers() {
		try {
			const response = await fetch('http://127.0.0.1:3000/admin/members/user');
			if (response.ok) {
				users = await response.json();
			} else {
				console.error('Failed to fetch users:', response.statusText);
			}
		} catch (error) {
			console.error('Error fetching users:', error);
		}
	}

	onMount(() => {
		fetchUsers();
	});
</script>

<Dashboard />
<div class="user-container">
	<h1>User List</h1>

	{#if users.length > 0}
		<table>
			<thead>
				<tr>
					<th>S.N</th>
					<th>First Name</th>
					<th>Last Name</th>
					<th>Username</th>
					<th>Email</th>
					<th>Total Reviews</th>
				</tr>
			</thead>
			<tbody>
				{#each users as user, count}
					<tr>
						<td>{count + 1}</td>
						<td>{user.firstname}</td>
						<td>{user.lastname}</td>
						<td>{user.username}</td>
						<td>{user.email}</td>
						<td>{user.total_reviews}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	{:else}
		<p>No users found.</p>
	{/if}
</div>

<style>
	@import '../body.css';
	.user-container {
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
		background-color: #116d73;
		font-weight: bold;
	}

	tr:nth-child(even) {
		background-color: #0e4165;
	}

	tr:hover {
		background-color: #036484;
	}
</style>
