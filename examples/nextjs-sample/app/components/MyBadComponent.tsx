'use client'

export function MyBadComponent() {
  return <div>Bad Component</div>
}

// This is wrong - server-side export in client component
export async function getServerSideProps() {
  return { props: {} }
}
