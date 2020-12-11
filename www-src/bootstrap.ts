let res = await import('./index').catch(e => console.error('Error importing `index.js`:', e));
export { res };