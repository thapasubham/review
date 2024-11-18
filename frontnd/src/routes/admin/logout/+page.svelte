<script>
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	let mesg = '';
	onMount(async () => {
		await logout();

		goto('/admin');
	});
	function deleteCookie(id) {
		localStorage.removeItem(id);
	}
	async function logout() {
		try {
			const token = localStorage.getItem('token');
			deleteCookie('token');
			alert('Logout Sucessfull');
		} catch (error) {
			console.error('Logout error:', error);
		}
	}
</script>

<p>Logging out...</p>
