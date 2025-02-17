import { describe, it, expect } from 'vitest'
import { PubkeyIndexMap } from '../../index.js'

describe('PubkeyIndexMap basics', () => {
  it('should initialize with size 0', () => {
    const map = new PubkeyIndexMap()
    expect(map.size).toBe(0)
  })

  it('should handle pubkey operations correctly', () => {
    const map = new PubkeyIndexMap()
    const pubkey = new Uint8Array(Buffer.alloc(48)).slice()
    const index = 42

    expect(map.has(pubkey)).toBe(false)

    const p = new Uint8Array(1000)
    expect(map.has(p.subarray(0, 48))).toBe(false)
    expect(() => map.has(Buffer.from('invalid'))).toThrow()
    expect(() => map.get(Buffer.from('invalid'))).toThrow()
    expect(() => map.set(Buffer.from('invalid'), 1)).toThrow()
    expect(() => map.delete(Buffer.from('invalid'))).toThrow()

    // Add a pubkey
    map.set(pubkey, index)
    map.set(pubkey, index + 1)
    map.set(pubkey, index)

    expect(map.size).toBe(1)
    expect(map.get(pubkey)).toBe(index)
    expect(map.get(pubkey.slice())).toBe(index)
    expect(map.has(pubkey)).toBe(true)
    expect(map.has(pubkey.slice())).toBe(true)

    // Add another pubkey
    const pubkey2 = new Uint8Array(Buffer.alloc(48, 1)).slice()
    const index2 = 43
    map.set(pubkey2, index2)

    expect(map.size).toBe(2)
    expect(map.get(pubkey2)).toBe(index2)

    // Remove a pubkey
    map.delete(pubkey)

    expect(map.size).toBe(1)
    expect(map.get(pubkey)).toBe(null)
    expect(map.get(pubkey2)).toBe(index2)

    // Clear the map
    map.clear()

    expect(map.size).toBe(0)
    expect(map.get(pubkey)).toBe(null)
    expect(map.get(pubkey2)).toBe(null)

    // Ensure different instances of the same pubkey are treated as the same
    const pubkey3 = pubkey.slice()
    const index3 = 44
    map.set(pubkey, index)
    map.set(pubkey3, index3)
    expect(map.get(pubkey)).toBe(index3)
    expect(map.get(pubkey3)).toBe(index3)
  })
})