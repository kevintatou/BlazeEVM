# Creating GitHub Issues from the Roadmap

This guide explains how to create GitHub issues from the roadmap defined in `ROADMAP.md`.

## Quick Start

Each section in the roadmap (e.g., "Issue 1.1", "Issue 2.3") is designed to be converted directly into a GitHub issue using the feature template at `.github/ISSUE_TEMPLATE/feature.md`.

## Step-by-Step Process

### 1. Choose an Issue from the Roadmap

Open `ROADMAP.md` and select an issue to implement. Issues are organized by phase, so you can follow them sequentially or pick based on priority.

**Example:** Issue 1.1: Implement `eth_blockNumber`

### 2. Create a New GitHub Issue

Navigate to the repository's Issues tab and click "New Issue". Select the "Feature: Micro-Issue" template.

### 3. Fill in the Template

Use the information from the roadmap section to fill in the GitHub issue template:

#### Title
Format: `[feat] <Summary from roadmap>`

**Example:** `[feat] Implement eth_blockNumber`

#### Summary
Copy the summary line from the roadmap.

**Example:** "Add JSON-RPC method to return the current block number."

#### Description
Copy the description from the roadmap. The roadmap provides detailed explanations of the behavior.

#### Acceptance Criteria
Copy the acceptance criteria checklist directly from the roadmap. These are the checkboxes that define "done".

#### Required Tests
Copy the "Required Tests" section from the roadmap. Each issue already specifies what tests are needed.

#### Scope Guardrails
Keep the standard guardrails from the template:
- Do NOT rename or move files unless specified.
- Do NOT refactor unrelated code.
- Do NOT introduce new dependencies (unless specified in the issue).
- Only modify the files needed to implement this behaviour.

#### Notes for Codex / AI
Copy the "Files to modify" or "Files to create" section from the roadmap. Add any example inputs/outputs if relevant.

### 4. Label and Assign

- Add the `feature` label (should be automatic from template)
- Consider adding phase labels like `phase-1`, `phase-2`, etc. for organization
- Assign to the appropriate developer or leave unassigned for others to pick up

## Example Issue Conversion

### From Roadmap (ROADMAP.md)

```markdown
### Issue 1.1: Implement `eth_blockNumber`
**Summary:** Add JSON-RPC method to return the current block number.

**Description:** 
Implement the `eth_blockNumber` method which returns the number of the most recent block. This requires integrating the `Chain` from core into the node's RPC handlers.

**Acceptance Criteria:**
- [ ] Add handler for `eth_blockNumber` in `node/src/rpc/eth.rs`
- [ ] Return current block number from chain state
- [ ] Return hex-encoded block number (e.g., "0x1" for block 1)
- [ ] Handle edge case of genesis block (should return "0x0")

**Required Tests:**
- [ ] Unit test for `eth_blockNumber` handler
- [ ] Integration test for `eth_blockNumber` endpoint
- [ ] Test with genesis block
- [ ] Test with multiple blocks in chain

**Files to modify:**
- `node/src/rpc/eth.rs` - Add handler logic
- `node/tests/integration_test.rs` - Add integration test
```

### To GitHub Issue

**Title:** `[feat] Implement eth_blockNumber`

**Body:**
```markdown
## Summary

Add JSON-RPC method to return the current block number.

## Description

Implement the `eth_blockNumber` method which returns the number of the most recent block. This requires integrating the `Chain` from core into the node's RPC handlers.

## Acceptance Criteria

- [ ] Add handler for `eth_blockNumber` in `node/src/rpc/eth.rs`
- [ ] Return current block number from chain state
- [ ] Return hex-encoded block number (e.g., "0x1" for block 1)
- [ ] Handle edge case of genesis block (should return "0x0")

## Required Tests

- [ ] Unit test for `eth_blockNumber` handler
- [ ] Integration test for `eth_blockNumber` endpoint
- [ ] Test with genesis block
- [ ] Test with multiple blocks in chain

## Scope Guardrails (IMPORTANT)

- Do NOT rename or move files unless specified.
- Do NOT refactor unrelated code.
- Do NOT introduce new dependencies.
- Only modify the files needed to implement this behaviour.

## Notes for Codex / AI

- Relevant modules/files:
  - `node/src/rpc/eth.rs` - Add handler logic
  - `node/tests/integration_test.rs` - Add integration test
- Example output:
  - Input: `{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}`
  - Output: `{"jsonrpc":"2.0","result":"0x1","id":1}` (for block 1)
```

## Issue Dependencies

Some issues depend on others being completed first. The roadmap generally orders issues to minimize dependencies, but here are key dependency chains:

### Phase 1 Dependencies
- Issues 1.2 and 1.3 should wait for Issue 1.4 (Shared State Management) OR can be implemented with a simpler state approach first

### Phase 2 Dependencies
- Issue 2.3 (`eth_sendRawTransaction`) depends on:
  - Issue 2.1 (Transaction structure)
  - Issue 2.2 (RLP encoding/decoding)
  
### Phase 3 Dependencies
- Issue 3.2 depends on Issue 3.1 (mining logic must exist before RPC method)
- Issue 3.3 depends on Issue 3.1 (need mining before auto-mining)

### Phase 4 Dependencies
- Issues 4.2-4.5 depend on Issue 4.3 (transaction storage must exist before querying)

## Prioritization Suggestions

While the roadmap is organized logically, here are suggested priorities for different use cases:

### For Basic Local Development Node
**Priority order:**
1. Phase 1: Issues 1.1-1.4 (basic queries and state)
2. Phase 2: Issues 2.1-2.3 (transaction submission)
3. Phase 3: Issues 3.1-3.3 (mining)
4. Phase 6: Issues 6.1-6.2 (test accounts and snapshots)

### For Production-Ready Node
**Priority order:**
1. Phases 1-5 (complete core functionality)
2. Phase 7 (configuration and logging)
3. Phase 8: Issue 8.1-8.2 (persistence)
4. Phase 9 (performance)

### For Testing/Development Tools
**Priority order:**
1. Phase 1: Issues 1.1-1.4
2. Phase 2: Issues 2.1-2.3
3. Phase 3: Issues 3.1-3.3
4. Phase 6: All (development tools)

### For Multi-Chain Research
**Priority order:**
1. Complete Phases 1-5 first
2. Phase 10: Issue 10.1 (design document)
3. Get feedback before implementing 10.2-10.4

## Tips for Implementation

1. **Start Small**: Begin with Phase 1 issues. They're simpler and build confidence.

2. **Follow the Template**: The feature template has guardrails that prevent scope creep.

3. **Test Thoroughly**: Each issue specifies required tests. Don't skip them.

4. **Reference Architecture**: Always check `ARCHITECTURE.md` before making structural changes.

5. **One Issue at a Time**: Resist the urge to combine issues. Keep PRs focused.

6. **Ask Questions**: If an issue description is unclear, ask for clarification before implementing.

7. **Update the Roadmap**: As issues are completed, consider adding checkmarks or status updates to `ROADMAP.md` to track progress.

## Tracking Progress

Consider adding phase labels to issues:
- `phase-1-essential-rpc`
- `phase-2-transactions`
- `phase-3-mining`
- etc.

This makes it easy to filter and see progress across different areas of development.

You can also create GitHub Projects or Milestones to track phases as a group.

## Questions?

If you have questions about any issue in the roadmap, please:
1. Check `ARCHITECTURE.md` for design principles
2. Look at existing code for patterns
3. Open a discussion or comment on the issue
4. Tag maintainers if needed

Happy building! 🚀
