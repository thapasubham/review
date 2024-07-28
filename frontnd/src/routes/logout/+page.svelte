<script>
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	let mesg = '';
	onMount(async () => {
		await logout();

		// Redirect to the home page
		goto('/');
	});
	function deleteCookie(name) {
		document.cookie = name + '=; expires=Thu, 01 Jan 1970 00:00:00 GMT; path=/';
	}
	async function logout() {
		try {
			// Make a request to the backend to invalidate the session or token
			const response = await fetch('http://127.0.1:3000/member/logout');
			const msg = await response.json();
			const mesg = msg.message;
			deleteCookie('token');
			alert(mesg);
			if (!response.ok) {
				throw new Error('Logout failed');
			}

			// Optionally clear any local storage or state here if needed
		} catch (error) {
			console.error('Logout error:', error);
		}
	}
</script>

<p>Logging out...</p>
