defmodule Aoc2022.MixProject do
  use Mix.Project

  def project do
    [
      app: :aoc2022,
      version: "0.1.0",
      deps: deps()
    ]
  end

  defp deps do
    [
      {:credo, "~> 1.6", only: [:dev, :test], runtime: false}
    ]
  end
end
