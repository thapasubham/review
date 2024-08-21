<script>
	export let firstname = '';
	export let lastname = '';
	export let username = '';
	export let password = '';
	export let c_password = '';
	export let phone = '';
	export let email = '';

	let touched = {
		firstname: false,
		lastname: false,
		username: false,
		password: false,
		c_password: false,
		phone: false,
		email: false
	};

	$: firstnameError = touched.firstname ? validateFirstname(firstname) : '';
	$: lastnameError = touched.lastname ? validateLastname(lastname) : '';
	$: usernameError = touched.username ? validateUsername(username) : '';
	$: phoneError = touched.phone ? validateNumber(phone) : '';
	$: emailError = touched.email ? validateEmail(email) : '';
	$: passwordError =
		touched.password || touched.c_password ? validatePassword(password, c_password) : '';

	function validateFirstname(field) {
		if (field === '') {
			return 'No Firstname was entered.\n';
		} else if (!/^[A-Z][a-z]*$/.test(field)) {
			return 'Enter Valid first Name.\n';
		}
		return '';
	}

	function validateLastname(field) {
		if (field === '') {
			return 'No Lastname was entered.\n';
		} else if (!/^[A-Z][a-z]*$/.test(field)) {
			return 'Enter Valid last Name.\n';
		}
		return '';
	}

	function validateUsername(field) {
		if (field === '') {
			return 'No Username was entered.\n';
		} else if (field.length < 5) {
			return 'Username must be at least 5 characters.\n';
		} else if (/[^a-z0-9_-]/.test(field)) {
			return 'Only a-z, A-Z, 0-9, - and _ allowed in Username.\n';
		}
		return '';
	}

	function validatePassword(password, c_password) {
		if (password !== c_password) {
			return "The password doesn't match.\n";
		} else if (password === '') {
			return 'No Password was entered.\n';
		} else if (password.length < 6) {
			return 'Password must be at least 6 characters.\n';
		} else if (!/[a-z]/.test(password) || !/[A-Z]/.test(password) || !/[0-9]/.test(password)) {
			return 'Password requires at least one lowercase letter, one uppercase letter, and one digit.\n';
		}
		return '';
	}

	function validateNumber(field) {
		if (isNaN(field)) {
			return 'No phone number was entered.\n';
		} else if (field.length < 10) {
			return 'Please enter a valid 10-digit phone number.\n';
		}
		return '';
	}

	function validateEmail(field) {
		if (field === '') {
			return 'No Email was entered.\n';
		} else if (!/\S+@\S+\.\S+/.test(field)) {
			return 'The Email is not valid.\n';
		}
		return '';
	}

	export async function register() {
		const response = await fetch('http://127.0.0.1:3000/member/register', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({
				firstname,
				lastname,
				username,
				phone,
				email,
				password
			})
		});

		const data = await response.json();
		if (data.response) {
			alert(data.response + '\n');
		}
	}

	function handleSubmit() {
		touched = {
			firstname: true,
			lastname: true,
			username: true,
			password: true,
			c_password: true,
			phone: true,
			email: true
		};
		if (
			!firstnameError &&
			!lastnameError &&
			!usernameError &&
			!phoneError &&
			!emailError &&
			!passwordError
		) {
			register();
		}
	}
</script>

<div>
	<div class="first-container">
		<h1>Register</h1>
		<div class="container">
			<form on:submit|preventDefault={handleSubmit} method="post">
				<label
					><input
						type="text"
						placeholder="Firstname"
						bind:value={firstname}
						on:blur={() => (touched.firstname = true)}
					/></label
				>
				<div id="firstname_error">{firstnameError}</div>

				<label
					><input
						type="text"
						placeholder="Lastname"
						bind:value={lastname}
						on:blur={() => (touched.lastname = true)}
					/></label
				>
				<div id="lastname_error">{lastnameError}</div>

				<label
					><input
						type="text"
						placeholder="Username"
						bind:value={username}
						on:blur={() => (touched.username = true)}
					/></label
				>
				<div id="username_error">{usernameError}</div>

				<label
					><input
						type="text"
						placeholder="Email"
						bind:value={email}
						on:blur={() => (touched.email = true)}
					/></label
				>
				<div id="email_error">{emailError}</div>

				<label
					><input
						type="text"
						placeholder="Phone Number"
						bind:value={phone}
						on:blur={() => (touched.phone = true)}
					/></label
				>
				<div id="phone_error">{phoneError}</div>

				<label
					><input
						type="password"
						placeholder="Password"
						bind:value={password}
						on:blur={() => (touched.password = true)}
					/></label
				>
				<div id="password_error">{passwordError}</div>

				<label
					><input
						type="password"
						placeholder="Confirm Password"
						bind:value={c_password}
						on:blur={() => (touched.c_password = true)}
					/></label
				>
				<div id="password_error">{passwordError}</div>

				<input type="submit" name="submit" />
				<a href="/login">Login</a>
			</form>
		</div>
	</div>
</div>

<style>
	@import '../../style.css';
	@import '../../body.css';
	.form_div {
		width: 80%;
		margin-left: auto;
		margin-right: auto;
		display: flex;
		justify-content: center;
		align-items: center;
	}
</style>
