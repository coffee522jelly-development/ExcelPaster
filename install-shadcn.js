import { spawn } from 'child_process';

const child = spawn('npx', ['shadcn-svelte@latest', 'init', '-o', '--cwd', '.'], { stdio: 'pipe' });

let buffer = '';

child.stdout.on('data', (data) => {
    const chunk = data.toString();
    buffer += chunk;
    console.log(chunk);

    if (buffer.includes('how would you like to')) {
         child.stdin.write('\x1B[B\x1B[B\r'); // Down Arrow, Down Arrow, Enter to select "Prompt me"
         buffer = '';
    } else if (buffer.includes('Which style would you like to use')) {
        child.stdin.write('\r');
        buffer = '';
    } else if (buffer.includes('Which base color would you like to use')) {
        child.stdin.write('\r');
        buffer = '';
    } else if (buffer.includes('Where is your global CSS file')) {
        child.stdin.write('\r');
        buffer = '';
    } else if (buffer.includes('Would you like to use CSS variables for colors')) {
        child.stdin.write('\r');
        buffer = '';
    } else if (buffer.includes('Where is your tailwind.config.js')) {
        child.stdin.write('\r');
        buffer = '';
    } else if (buffer.includes('Configure the import alias for components')) {
        child.stdin.write('\r');
        buffer = '';
    } else if (buffer.includes('Configure the import alias for utils')) {
        child.stdin.write('\r');
        buffer = '';
    } else if (buffer.includes('Are you using React Server Components')) {
        child.stdin.write('\r');
        buffer = '';
    }
});

child.stderr.on('data', (data) => {
    console.error(data.toString());
});

child.on('close', (code) => {
    console.log(`Process exited with code ${code}`);
});
