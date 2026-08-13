<script lang="ts">
  import Suite from './lib/Suite.svelte';
  type Role = 'orchestrator'|'bank'|'customer'|'airline'|'retailer'|'risk'|'investor';
  type Section = 'overview'|'suite'|'journey'|'economics'|'architecture'|'controls'|'roadmap';

  const roles: Record<Role, any> = {
    orchestrator:{label:'Platform operator',abbr:'OP',color:'#69e7c1',objective:'Make the network reliable and repeatable',gives:['Workflow infrastructure','Partner integrations','Evidence & analytics'],gets:['Annual platform fees','Usage revenue','Defensible network data'],kpis:['99.95% uptime','<0.1% settlement errors','60%+ gross margin'],decision:'Can every participant win without the platform taking credit risk?'},
    bank:{label:'Bank executive',abbr:'BK',color:'#8fb8ff',objective:'Improve completion, retention and lifetime value',gives:['Regulated loan product','Servicing & reporting','Customer distribution'],gets:['Higher completion','Lower servicing leakage','Graduation pipeline'],kpis:['+12% completion','-18% early exits','3.2× program LTV'],decision:'Does incremental value exceed rewards, compliance and integration cost?'},
    customer:{label:'Credit builder',abbr:'CB',color:'#f6c66d',objective:'Build credit while receiving useful progress rewards',gives:['On-time repayments','Consent & engagement','Savings habit'],gets:['Credit history','Milestone benefits','Hardship support'],kpis:['12 on-time payments','Benefit clarity','Savings retained'],decision:'Is the product fair, understandable and useful every month?'},
    airline:{label:'Airline loyalty',abbr:'AL',color:'#b99cff',objective:'Acquire future frequent travelers efficiently',gives:['Miles, passes & status boosts','Wholesale inventory','Campaign funding'],gets:['New loyalty members','Incremental bookings','Brand preference'],kpis:['Activated members','Route contribution','Redemption margin'],decision:'Does bank distribution beat paid acquisition on contribution margin?'},
    retailer:{label:'Retail partner',abbr:'RT',color:'#ff9887',objective:'Convert bank customers into incremental shoppers',gives:['Vouchers & access','Offer inventory','Redemption data'],gets:['Basket uplift','Repeat purchase','Measured attribution'],kpis:['Incremental basket','Repeat rate','Campaign ROAS'],decision:'Are redemptions incremental rather than subsidizing existing spend?'},
    risk:{label:'Risk & compliance',abbr:'RC',color:'#ef799c',objective:'Protect customers and preserve regulatory control',gives:['Approval gates','Monitoring standards','Stop authority'],gets:['Auditability','Data minimization','Controlled third parties'],kpis:['0 material breaches','Complaint parity','100% traceability'],decision:'Can the bank explain, monitor and stop every customer-impacting action?'},
    investor:{label:'Founder / investor',abbr:'FI',color:'#9bd67a',objective:'Build durable recurring revenue with limited balance-sheet risk',gives:['Capital & governance','Commercial network','Execution runway'],gets:['Contracted ARR','Expansion revenue','Strategic moat'],kpis:['$1m pilot ARR','120%+ NRR','<18-month CAC payback'],decision:'Can pilots convert into multi-bank, multi-partner infrastructure?'},
  };

  const steps = [
    {n:'01',title:'Design & approve',owner:'Bank + Risk',body:'Define eligibility, milestones, reward funding, disclosures, hardship rules and stop conditions.',proof:'Approved product specification'},
    {n:'02',title:'Enroll & consent',owner:'Customer + Bank',body:'Customer opens the bank’s credit-builder loan and separately accepts clear reward-program terms.',proof:'Versioned consent record'},
    {n:'03',title:'Verify milestone',owner:'Platform',body:'Bank servicing event arrives through a signed API. Rules engine validates an on-time payment.',proof:'Immutable audit event'},
    {n:'04',title:'Unlock benefit',owner:'Partner',body:'A fixed, funded entitlement is released—never represented as loan principal, collateral or investment yield.',proof:'Entitlement + funding record'},
    {n:'05',title:'Intervene early',owner:'Bank',body:'Risk signals route the customer to bank-approved hardship options before a missed payment.',proof:'Customer-authorized action'},
    {n:'06',title:'Settle & learn',owner:'All parties',body:'Redemptions reconcile, economics attribute, outcomes compare to a control cohort, and rules improve.',proof:'Board-ready outcome report'},
  ];

  const controls = [
    ['Product boundary','The platform does not lend, hold principal, make credit decisions or promise investment returns.','Bank legal'],
    ['Consumer fairness','Plain-language value, expiry and forfeiture rules; no distressed secondary-market sale.','Compliance'],
    ['Fair lending','Outcome monitoring by protected and vulnerable segments; human escalation.','Model risk'],
    ['Partner funding','Every entitlement maps to an authorized inventory and settlement obligation.','Finance'],
    ['Data governance','Consent, minimization, purpose limitation, retention and revocation controls.','Privacy'],
    ['Operational resilience','Idempotent events, reconciliation, incident playbooks, exit and portability plan.','Technology risk'],
  ];

  let role: Role = 'orchestrator';
  let section: Section = 'overview';
  let loanVolume = 25000;
  let monthlyFee = 2.4;
  let completionLift = 12;
  let partnerContribution = 18;
  let saved = false;
  const nav: {id:Section;label:string}[] = [{id:'overview',label:'System map'},{id:'suite',label:'Business suite'},{id:'journey',label:'Operating journey'},{id:'economics',label:'Economics lab'},{id:'architecture',label:'Architecture'},{id:'controls',label:'Controls'},{id:'roadmap',label:'Implementation'}];
  $: r = roles[role];
  $: arr = loanVolume * monthlyFee * 12;
  $: completionValue = loanVolume * (completionLift/100) * 145;
  $: partnerValue = loanVolume * partnerContribution;
  $: ecosystemValue = arr + completionValue + partnerValue;

  async function saveScenario(){
    try { await fetch('/api/scenarios',{method:'POST',headers:{'content-type':'application/json'},body:JSON.stringify({role,loan_volume:loanVolume,monthly_fee:monthlyFee,completion_lift:completionLift,partner_contribution:partnerContribution})}); } catch {}
    saved=true; setTimeout(()=>saved=false,1800);
  }
</script>

<svelte:head><title>CreditFlow Atlas — stakeholder operating model</title><meta name="description" content="An interactive end-to-end operating model for a bank-grade credit builder rewards platform."/></svelte:head>

<div class="shell">
  <aside>
    <div class="brand"><span class="mark">Cƒ</span><div><b>CreditFlow</b><small>ATLAS / 01</small></div></div>
    <p class="eyebrow">Choose a perspective</p>
    <div class="roles">
      {#each Object.entries(roles) as [key,item]}
        <button class:active={role===key} onclick={()=>role=key as Role} style={`--accent:${item.color}`}><i>{item.abbr}</i><span>{item.label}<small>{item.objective}</small></span></button>
      {/each}
    </div>
    <div class="side-note"><span>Design principle</span><p>Reward progress. Intervene early. Never turn customer distress into marketplace inventory.</p></div>
  </aside>

  <main>
    <header><div><span class="live"><i></i> OPERATING MODEL</span><h1>One system.<br/><em>Seven truths.</em></h1></div><div class="header-meta"><span>Bank-grade concept</span><b>v1.0</b></div></header>
    <nav>{#each nav as item}<button class:active={section===item.id} onclick={()=>section=item.id}>{item.label}</button>{/each}</nav>

    {#if section==='overview'}
      <section class="hero-grid">
        <div class="role-card" style={`--role:${r.color}`}><span class="kicker">YOUR LENS / {r.abbr}</span><h2>{r.label}</h2><p>{r.objective}</p><div class="decision"><span>DECISION TO PROVE</span>{r.decision}</div></div>
        <div class="value-map">
          <div class="orbit">
            <div class="core"><small>ORCHESTRATION</small><b>CreditFlow<br/>Platform</b><span>rules • evidence • settlement</span></div>
            {#each Object.entries(roles).slice(1,6) as [key,item],i}<button class:selected={role===key} class={`node n${i+1}`} onclick={()=>role=key as Role} style={`--node:${item.color}`}><i>{item.abbr}</i><span>{item.label}</span></button>{/each}
          </div>
          <div class="legend"><span><i class="mint"></i>value</span><span><i class="amber"></i>evidence</span><span><i class="blue"></i>control</span></div>
        </div>
        <div class="exchange"><div><span>CONTRIBUTES</span>{#each r.gives as x}<p>↗ {x}</p>{/each}</div><div><span>RECEIVES</span>{#each r.gets as x}<p>↙ {x}</p>{/each}</div></div>
      </section>
      <section class="metric-row">{#each r.kpis as metric,i}<article><span>0{i+1}</span><b>{metric}</b><small>{i===0?'PRIMARY OUTCOME':i===1?'HEALTH SIGNAL':'VALUE PROOF'}</small></article>{/each}</section>
      <div class="narrative"><span>THE BUSINESS IN ONE SENTENCE</span><p>A white-label bank platform converts verified repayment progress into funded partner benefits, intervenes before hardship becomes delinquency, and proves incremental value to every participant through a shared evidence layer.</p></div>
    {:else if section==='suite'}
      <Suite {role} roleLabel={r.label} roleColor={r.color}/>
    {:else if section==='journey'}
      <section class="section-head"><span>END-TO-END WORKFLOW</span><h2>From product approval to measurable outcome.</h2><p>Every step has an accountable owner, a customer-facing event, and evidence that survives audit.</p></section>
      <div class="timeline">{#each steps as step}<article><span class="num">{step.n}</span><div><small>{step.owner}</small><h3>{step.title}</h3><p>{step.body}</p><b>✓ {step.proof}</b></div></article>{/each}</div>
    {:else if section==='economics'}
      <section class="section-head"><span>SCENARIO ENGINE</span><h2>Make the value pool explicit.</h2><p>Illustrative economics only. Replace assumptions with pilot evidence before any commercial commitment.</p></section>
      <div class="lab"><div class="sliders">
        <label><span>Active accounts <b>{loanVolume.toLocaleString()}</b></span><input type="range" min="1000" max="100000" step="1000" bind:value={loanVolume}/></label>
        <label><span>Platform fee / account / month <b>${monthlyFee.toFixed(2)}</b></span><input type="range" min="0.5" max="8" step="0.1" bind:value={monthlyFee}/></label>
        <label><span>Completion improvement <b>{completionLift}%</b></span><input type="range" min="0" max="30" bind:value={completionLift}/></label>
        <label><span>Partner contribution / account <b>${partnerContribution}</b></span><input type="range" min="0" max="80" bind:value={partnerContribution}/></label>
        <button class="save" onclick={saveScenario}>{saved?'SCENARIO SAVED ✓':'SAVE SCENARIO'}</button>
      </div><div class="outcomes"><span>ANNUAL VALUE POOL</span><strong>${Math.round(ecosystemValue).toLocaleString()}</strong><div><article><small>Platform ARR</small><b>${Math.round(arr).toLocaleString()}</b></article><article><small>Bank completion value</small><b>${Math.round(completionValue).toLocaleString()}</b></article><article><small>Partner-funded value</small><b>${Math.round(partnerValue).toLocaleString()}</b></article></div><p>North-star test: total incremental value must exceed reward, servicing, integration, fraud, compliance and capital costs.</p></div></div>
      <div class="revenue-stack">{#each [['Platform license','Predictable annual access'],['Usage commitment','Reserved account capacity'],['Implementation','Integration and controls'],['Campaign fee','Measured partner activation'],['Analytics add-on','Cohorts and experiments']] as item,i}<article><i>0{i+1}</i><b>{item[0]}</b><span>{item[1]}</span></article>{/each}</div>
    {:else if section==='architecture'}
      <section class="section-head"><span>REFERENCE ARCHITECTURE</span><h2>Regulated edges. Neutral orchestration core.</h2><p>The bank remains system of record for credit. Partners remain system of record for benefit inventory.</p></section>
      <div class="architecture"><div class="lane"><span>BANK CONTROL PLANE</span><div>Loan origination</div><div>Servicing events</div><div>Credit reporting</div><div>Hardship actions</div></div><div class="platform-lane"><span>CREDITFLOW PLATFORM</span><div class="arch-core"><b>API Gateway</b><b>Eligibility & rules</b><b>Entitlement ledger</b><b>Consent vault</b><b>Settlement</b><b>Evidence warehouse</b></div><small>Signed events → idempotent processing → reconciled outcomes</small></div><div class="lane"><span>PARTNER CONTROL PLANE</span><div>Offer inventory</div><div>Member enrollment</div><div>Redemption</div><div>Attribution</div></div></div>
      <div class="tech"><article><span>WEB</span><b>Svelte 5</b><p>Role-guided presentation and operating cockpit.</p></article><article><span>CORE</span><b>Axum / Rust</b><p>Typed APIs, rules, settlement and audit events.</p></article><article><span>DATA</span><b>rusqlite</b><p>Pilot persistence with migration path to managed Postgres.</p></article><article><span>DESKTOP</span><b>Tauri 2</b><p>Secure offline partner presentation shell.</p></article></div>
    {:else if section==='controls'}
      <section class="section-head"><span>TRUST ARCHITECTURE</span><h2>Controls are part of the product.</h2><p>No participant can create customer-impacting value without authorization, evidence and a reversal path.</p></section>
      <div class="controls">{#each controls as c,i}<article><span>GATE {i+1}</span><h3>{c[0]}</h3><p>{c[1]}</p><b>Owner · {c[2]}</b></article>{/each}</div>
      <div class="red-line"><b>NON-NEGOTIABLE BOUNDARY</b><p>No public secondary market, no yield-bearing token, no claim that benefits collateralize debt, and no sale of distressed customers’ future value to professional buyers.</p></div>
    {:else}
      <section class="section-head"><span>PHASED IMPLEMENTATION</span><h2>Earn complexity through evidence.</h2><p>Each phase has a commercial gate. Do not scale what has not improved customer outcomes.</p></section>
      <div class="phases">{#each [
        ['0 · VALIDATE','4–6 weeks','Bank data baseline, customer research, partner inventory design, legal perimeter.','Signed pilot thesis + stop conditions'],
        ['1 · PILOT BUILD','8–12 weeks','One bank, one partner, fixed milestone rules, consent, audit and settlement.','Security + product approval'],
        ['2 · CONTROLLED LIVE','6–12 months','Limited cohort plus control group, hardship routing and outcome reporting.','Positive risk-adjusted unit economics'],
        ['3 · MULTI-PARTNER','3–6 months','Campaign marketplace, configurable rules, automated reconciliation.','Repeatable partner onboarding'],
        ['4 · SCALE','Ongoing','Multi-bank tenancy, enterprise controls, geographic expansion.','Contracted expansion ARR']
      ] as p,i}<article><i>{i+1}</i><div><span>{p[0]} · {p[1]}</span><h3>{p[2]}</h3><b>GATE → {p[3]}</b></div></article>{/each}</div>
      <div class="loop"><span>THE EXECUTION LOOP</span>{#each ['Hypothesis','Configure','Launch','Measure','Review risk','Expand or stop'] as x,i}<div><i>{i+1}</i>{x}</div>{/each}</div>
    {/if}
    <footer><span>CREDITFLOW ATLAS</span><p>Strategic product model · Not legal, financial or regulatory advice</p><b>{new Date().getFullYear()}</b></footer>
  </main>
</div>
