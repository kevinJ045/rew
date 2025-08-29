"no-compile"
if (!rew.extensions.has('testing')) {
  // ---- Test Registry ----
  const testRegistry = {
    groups: new Map(),
    addGroup(name) {
      if (!this.groups.has(name)) this.groups.set(name, []);
    },
    addTest(group, name, fn) {
      if (!this.groups.has(group)) this.addGroup(group);
      this.groups.get(group).push({ name, fn });
    },
    async run(groupName) {
      const tests = this.groups.get(groupName);
      if (!tests) {
        return;
      }

      rew.prototype.io.prototype.out.print(`\n▶ Running group: ${groupName}`);
      let passed = 0, failed = 0;
      for (const { name, fn } of tests) {
        try {
          await fn();
          rew.prototype.io.prototype.out.print(`   ✅ ${name}`);
          passed++;
        } catch (err) {
          rew.prototype.io.prototype.out.print(`   ❌ ${name}`);
          rew.prototype.io.prototype.out.print("      ", err.message || err);
          failed++;
        }
      }
      rew.prototype.io.prototype.out.print(`\nGroup '${groupName}' finished → Passed: ${passed}, Failed: ${failed}\n`);
    }
  };


  rew.extensions.add('testing', (Deno) => rew.extensions.createClass({
    _namespace() {
      return "testing";
    },
    describe(groupName, fn) {
      if(!Deno.tests)
      testRegistry.addGroup(groupName);

      function it(testName, fn) {
        testRegistry.addTest(groupName, testName, fn);
      }

      fn(it, this.assert);
    },

    assert(cond, msg = "Assertion failed") {
      if (!cond) throw new Error(msg);
    },

    assert_eq(a, b, msg) {
      if (a !== b) {
        throw new Error(msg || `Assertion failed: expected '${a}' === '${b}'`);
      }
    },

    assert_neq(a, b, msg) {
      if (a === b) {
        throw new Error(msg || `Assertion failed: expected '${a}' !== '${b}'`);
      }
    },

    assert_gt(a, b, msg) {
      if (!(a > b)) {
        throw new Error(msg || `Assertion failed: expected ${a} > ${b}`);
      }
    },

    assert_gte(a, b, msg) {
      if (!(a >= b)) {
        throw new Error(msg || `Assertion failed: expected ${a} >= ${b}`);
      }
    },

    assert_lt(a, b, msg) {
      if (!(a < b)) {
        throw new Error(msg || `Assertion failed: expected ${a} < ${b}`);
      }
    },

    assert_lte(a, b, msg) {
      if (!(a <= b)) {
        throw new Error(msg || `Assertion failed: expected ${a} <= ${b}`);
      }
    },

    assert_deep_eq(a, b, msg) {
      const ja = JSON.stringify(a);
      const jb = JSON.stringify(b);
      if (ja !== jb) {
        throw new Error(msg || `Assertion failed: deep equal expected ${ja} === ${jb}`);
      }
    },

    assert_match(str, regex, msg) {
      if (!regex.test(str)) {
        throw new Error(msg || `Assertion failed: '${str}' does not match ${regex}`);
      }
    },

    async run(groupName){
      return await testRegistry.run(groupName);
    },

    async runAll(groups){
      if(!groups || !groups.length){
        groups = testRegistry.groups.keys();
      }
      
      for(let i of groups){
        await testRegistry.run(i);
      }
    }

  }));

}