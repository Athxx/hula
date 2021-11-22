<script lang="ts">
	import {locale} from 'svelte-i18n';
	import {Link} from 'svelte-routing';
	import {get} from 'svelte/store';

	export let title: string;
	let collapsed = true;
	let expanded = false;

	locale.subscribe((lang) => {
		localStorage.setItem('locale', lang);
	});

	const isCurrentLanguage = (pattern: RegExp): boolean => {
		return pattern.test(<string>get(locale));
	};
	const changeLanguage = (lang: string): void => {
		locale.set(lang);
		expanded = false;
	};
</script>

<header>
	<nav
		class="navbar navbar-expand-sm navbar-toggleable-sm navbar-light bg-white border-bottom box-shadow mb-3"
	>
		<div class="container">
			<a class="navbar-brand" href="/">{title}</a>
			<button
				aria-expanded={!collapsed}
				aria-label="Toggle navigation"
				class="navbar-toggler"
				data-target=".navbar-collapse"
				data-toggle="collapse"
				on:click={() => (collapsed = !collapsed)}
				type="button"
			>
				<span class="navbar-toggler-icon"/>
			</button>
			<div
				class={`navbar-collapse collapse d-sm-inline-flex justify-content-end ${
          collapsed ? '' : 'show'
        }`}
				role="menu"
			>
				<ul class="navbar-nav flex-grow">
					<li class="nav-item">
						<Link class="nav-link" to="/">Home</Link>
					</li>
					<li class="nav-item">
						<Link class="nav-link" to="/counter">Counter</Link>
					</li>
					<li class="nav-item">
						<Link class="nav-link" to="/todo-list">Todo</Link>
					</li>
					<li class="nav-item dropdown">
						<button
							aria-expanded={expanded}
							aria-label="Toggle Languages"
							class={`btn dropdown-toggle ${expanded ? 'show' : ''}`}
							data-bs-auto-close="true"
							data-bs-toggle="dropdown"
							id="i18nDropdown"
							on:click={() => (expanded = !expanded)}
						>
							<i class="bi bi-globe"/>
						</button>
					</li>
				</ul>
			</div>
		</div>
	</nav>
</header>
