import React from 'react'

declare module 'react' {
  // augment CSSProperties here
  interface CSSProperties {
    '--value'?: string | number
  }
}
