<script>
  import { Button } from "$lib/components/ui/button";
  import {
    Card,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
    CardAction,
  } from "$lib/components/ui/card";
</script>

<Card class="text-center m-auto max-w-lg">
  <CardHeader>
    <CardTitle><h1>You owned it!</h1></CardTitle>
  </CardHeader>
  <CardContent>
    <CardDescription
      >Workstation helps you configure your development environment more easily!</CardDescription
    >
  </CardContent>
  <CardFooter class="flex items-center justify-center">
    <CardAction>
      <a href="/ssh">
        <Button>Get Started</Button>
      </a>
    </CardAction>
  </CardFooter>
</Card>
