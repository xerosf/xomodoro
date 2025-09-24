<script lang="ts">
	import { getCurrentWindow } from '@tauri-apps/api/window';

	const appWindow = getCurrentWindow();

	async function onMinimize() {
		try {
			await appWindow.minimize();
		} catch (err) {
			console.error('Failed to minimize window', err);
		}
	}

	async function onClose() {
		try {
			await appWindow.close();
		} catch (err) {
			console.error('Failed to close window', err);
		}
	}
</script>

<div class="titlebar" data-tauri-drag-region>
    <div class="title">
        <img src="/icon.png" alt="App Icon">
        xomodoro
    </div>
	<div class="controls" aria-label="Window controls">
		<button
			class="btn"
			on:click|stopPropagation={onMinimize}
			aria-label="Minimize"
			title="Minimize"
		>
			<svg width="32" height="32" viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M7 16C6.44772 16 6 16.4477 6 17C6 17.5523 6.44772 18 7 18H25C25.5523 18 26 17.5523 26 17C26 16.4477 25.5523 16 25 16H7Z" fill="currentColor"/>
            </svg>
		</button>

		<button
			class="btn close"
			on:click|stopPropagation={onClose}
			aria-label="Close"
			title="Close"
		>
			<svg width="32" height="32" viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M5.7948 5.79474C6.16103 5.42852 6.74089 5.40572 7.13367 5.72638L7.20886 5.79474L15.9999 14.5858L24.7909 5.79474L24.8671 5.72638C25.2598 5.40584 25.8397 5.42855 26.2059 5.79474C26.5718 6.16096 26.5948 6.74094 26.2743 7.13361L26.2059 7.2088L17.4139 15.9998L26.205 24.7908C26.5955 25.1814 26.5955 25.8144 26.205 26.2049C25.8144 26.5954 25.1814 26.5954 24.7909 26.2049L15.9999 17.4139L7.20886 26.2049C6.81834 26.5954 6.18532 26.5954 5.7948 26.2049C5.40446 25.8144 5.40434 25.1813 5.7948 24.7908L14.5858 15.9998L5.7948 7.2088L5.72644 7.13361C5.40577 6.74083 5.42857 6.16097 5.7948 5.79474Z" fill="currentColor"/>
            </svg>
		</button>
	</div>
</div>

<style>
	.titlebar {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0;
		-webkit-app-region: drag;
		z-index: 10;
		user-select: none;
	}

    .title {
        display: flex;
        font-size: 0.875rem;
        flex-direction: row;
        align-items: center;
        gap: 8px;
        padding-left: 8px;
    }

    .title img {
        height: 16px;
        width: 16px;
    }

	.controls {
		display: flex;
		align-items: stretch;
		height: 100%;
		-webkit-app-region: no-drag;
	}

	.btn {
		appearance: none;
		border: none;
		margin: 0;
		padding: 0 12px;
		height: 100%;
		width: 46px;
		display: grid;
		place-items: center;
		background: transparent;
		color: inherit;
		cursor: default;
		outline: none;
        transition: background 0.2s ease;
	}

	.btn svg {
		width: 16px;
		height: 16px;
		pointer-events: none;
	}

	/* Hover/active states to mimic native behavior */
	.btn:hover { background: rgba(255, 255, 255, 0.08); }
	.btn:active { background: rgba(255, 255, 255, 0.14); }

	.btn.close:hover { background: #e81123; }
	.btn.close:active { background: #c50f1f; }
</style>
