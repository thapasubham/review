<script>
	let file;
	let responseMessage = '';

	async function handleFileUpload() {
		try {
			const formData = new FormData();
			formData.append('file', file[0]);

			const response = await fetch('http://localhost:3000/upload', {
				method: 'POST',
				body: formData,
				headers: {
					Accept: 'application/json'
				}
			});

			if (!response.ok) {
				throw new Error(`Error: ${response.statusText}`);
			}

			const result = await response.text();
			responseMessage = result;
		} catch (error) {
			responseMessage = `Upload failed: ${error.message}`;
		}
	}
</script>

<form on:submit|preventDefault={handleFileUpload}>
	<input type="file" bind:files={file} />
	<button type="submit">Upload</button>
</form>

<!-- {#if responseMessage}
	<p>{responseMessage}</p>
{/if} -->
<style>
</style>
